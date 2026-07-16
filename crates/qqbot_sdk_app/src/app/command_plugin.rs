use crate::app::config::AppConfig;
use crate::app::{ApiClient, App, Depend, DependStore};
use crate::Plugin;
use qqbot_sdk_commands::{wrap_command_handle_fn, CommandDef, CommandsStore, ReplyingMessage};
use qqbot_sdk_core::events::c2c::event_type::C2cEventTypeKind;
use qqbot_sdk_core::events::c2c::models::C2cMessage;
use qqbot_sdk_core::events::common::CommonMessage;
use qqbot_sdk_core::events::group::event_type::GroupEventTypeKind;
use qqbot_sdk_core::events::group::models::GroupAtMessage;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info, warn};

/// 将命令表适配为消息事件处理器的内置插件。
pub(crate) struct CommandPlugin;

impl CommandPlugin {
    /// 从应用配置收集手动注册的命令。
    pub(crate) fn from_config(config: &AppConfig, app: &App) -> Self {
        let mut commands = HashMap::new();

        for command in inventory::iter::<CommandDef> {
            let replaced = commands.insert(command.prefix, wrap_command_handle_fn(command.handler));
            if !config.ignore_checking && replaced.is_some() {
                panic!("Command {:?} duplicated", command.prefix);
            }
        }

        for (prefix, handler) in &config.commands {
            let replaced = commands.insert(*prefix, Arc::clone(handler));
            if !config.ignore_checking && replaced.is_some() {
                panic!("Command {:?} duplicated", prefix);
            }
        }

        let replaced = app.depend_store.insert(CommandsStore::new(commands));
        if !config.ignore_checking && replaced.is_some() {
            panic!(
                "Depend {:?} duplicated",
                std::any::type_name::<CommandsStore>()
            );
        }

        Self
    }

    async fn handle_c2c(
        message: C2cMessage,
        api: Depend<ApiClient>,
        commands: Depend<CommandsStore>,
        dependencies: DependStore,
    ) {
        if let Some(reply) = Self::handle_message(&message, &commands, &dependencies).await {
            let body =
                reply.to_request(Some(message.id.clone()), Some(message.msg_seq.unwrap_or(1)));
            let result = api
                .c2c_messages()
                .send_typed(&message.author.user_openid, &body)
                .await;
            info!("send c2c message result: {:?}", result);
        }
    }

    async fn handle_group(
        message: GroupAtMessage,
        api: Depend<ApiClient>,
        commands: Depend<CommandsStore>,
        dependencies: DependStore,
    ) {
        if let Some(reply) = Self::handle_message(&message, &commands, &dependencies).await {
            let body =
                reply.to_request(Some(message.id.clone()), Some(message.msg_seq.unwrap_or(1)));
            let result = api
                .group_messages()
                .send_typed(&message.group_openid, &body)
                .await;
            info!("send group message result: {:?}", result);
        }
    }

    async fn handle_message(
        message: &dyn CommonMessage,
        commands: &CommandsStore,
        dependencies: &DependStore,
    ) -> Option<ReplyingMessage> {
        let content = message.get_content().as_deref()?;
        let command = content.split_whitespace().next()?;
        let Some(handler) = commands.get(command) else {
            warn!("未知指令: {}", content);
            return None;
        };

        match handler(message, dependencies).await {
            Ok(reply) => reply,
            Err(err) => {
                error!("处理指令{}出错: {}", content, err);
                None
            }
        }
    }
}

impl Plugin for CommandPlugin {
    fn register(&self, app: &App) {
        app.registe_event_handler(C2cEventTypeKind::C2cMessageCreate, Self::handle_c2c);

        app.registe_event_handler(GroupEventTypeKind::GroupAtMessageCreate, Self::handle_group);
    }
}
