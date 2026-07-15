use crate::app::config::AppConfig;
use crate::app::{ApiClient, App};
use crate::Plugin;
use qqbot_sdk_commands::{
    wrap_command_handle_fn, CommandDef, CommandsStore, ContextStore, ReplyingMessage,
};
use qqbot_sdk_core::events::c2c::event_type::C2cEventTypeKind;
use qqbot_sdk_core::events::c2c::models::C2cMessage;
use qqbot_sdk_core::events::common::CommonMessage;
use qqbot_sdk_core::events::group::event_type::GroupEventTypeKind;
use qqbot_sdk_core::events::group::models::GroupAtMessage;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info, warn};

#[derive(Clone)]
/// 将命令表适配为消息事件处理器的内置插件。
pub(crate) struct CommandPlugin {
    api: Arc<ApiClient>,
    commands: CommandsStore,
    context: ContextStore,
}

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

        Self {
            api: app.get_api_client(),
            commands: CommandsStore::new(commands),
            context: app.dependency_container.clone(),
        }
    }

    async fn handle_c2c(&self, message: C2cMessage) {
        if let Some(reply) = self.handle_message(&message).await {
            let body =
                reply.to_request(Some(message.id.clone()), Some(message.msg_seq.unwrap_or(1)));
            let result = self
                .api
                .c2c_messages()
                .send_typed(&message.author.user_openid, &body)
                .await;
            info!("send c2c message result: {:?}", result);
        }
    }

    async fn handle_group(&self, message: GroupAtMessage) {
        if let Some(reply) = self.handle_message(&message).await {
            let body =
                reply.to_request(Some(message.id.clone()), Some(message.msg_seq.unwrap_or(1)));
            let result = self
                .api
                .group_messages()
                .send_typed(&message.group_openid, &body)
                .await;
            info!("send group message result: {:?}", result);
        }
    }

    async fn handle_message(&self, message: &dyn CommonMessage) -> Option<ReplyingMessage> {
        let content = message.get_content().as_deref()?;
        let command = content.split_whitespace().next()?;
        let Some(handler) = self.commands.get(command) else {
            warn!("未知指令: {}", content);
            return None;
        };

        match handler(message, &self.context).await {
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
        let c2c_plugin = self.clone();
        app.registe_event_handler(
            C2cEventTypeKind::C2cMessageCreate,
            move |message: C2cMessage| {
                let plugin = c2c_plugin.clone();
                async move { plugin.handle_c2c(message).await }
            },
        );

        app.registe_event_handler(GroupEventTypeKind::GroupAtMessageCreate, {
            let plugin = self.clone();
            move |message: GroupAtMessage| {
                let plugin = plugin.clone();
                async move { plugin.handle_group(message).await }
            }
        });
    }
}
