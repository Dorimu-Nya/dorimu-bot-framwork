use qqbot_sdk_app::ApiClient;
use qqbot_sdk_commands::{
    wrap_command_handle_fn, CommandDef, CommandHandler, CommandsStore, DynCommandHandleFn,
    ReplyingMessage,
};
use qqbot_sdk_core::events::c2c::event_type::C2cEventTypeKind;
use qqbot_sdk_core::events::c2c::models::C2cMessage;
use qqbot_sdk_core::events::group::event_type::GroupEventTypeKind;
use qqbot_sdk_core::events::group::models::GroupAtMessage;
use qqbot_sdk_runtime::{Depend, DependStore, Plugin, PluginRegistrar};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info, warn};

/// 将命令表适配为消息事件处理器的插件。
///
/// 只有通过 [`qqbot_sdk_app::AppConfig::with_plugin`] 显式加载后，命令才会执行。
pub struct CommandPlugin {
    commands: HashMap<&'static str, DynCommandHandleFn>,
    ignore_checking: bool,
}

impl Default for CommandPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandPlugin {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            ignore_checking: false,
        }
    }

    /// 配置是否忽略重复命令和依赖检查。
    pub fn ignore_checking(mut self, ignore: bool) -> Self {
        self.ignore_checking = ignore;
        self
    }

    /// 手动注册一个命令处理器。
    pub fn with_command<H, Args, Kind>(mut self, prefix: &'static str, handler: H) -> Self
    where
        H: CommandHandler<Args, Kind>,
    {
        let replaced = self.commands.insert(prefix, handler.into_dyn());
        if !self.ignore_checking && replaced.is_some() {
            panic!("Command {:?} duplicated", prefix);
        }
        self
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
        message: &dyn qqbot_sdk_commands::CommonMessage,
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
    fn register(&self, registrar: &PluginRegistrar<'_>) {
        let mut commands = HashMap::new();

        for command in inventory::iter::<CommandDef> {
            let replaced = commands.insert(command.prefix, wrap_command_handle_fn(command.handler));
            if !self.ignore_checking && replaced.is_some() {
                panic!("Command {:?} duplicated", command.prefix);
            }
        }

        for (prefix, handler) in &self.commands {
            let replaced = commands.insert(*prefix, Arc::clone(handler));
            if !self.ignore_checking && replaced.is_some() {
                panic!("Command {:?} duplicated", prefix);
            }
        }

        let replaced = registrar.insert_dependency(CommandsStore::new(commands));
        if !self.ignore_checking && replaced.is_some() {
            panic!(
                "Depend {:?} duplicated",
                std::any::type_name::<CommandsStore>()
            );
        }

        registrar.register_event_handler(C2cEventTypeKind::C2cMessageCreate, Self::handle_c2c);
        registrar
            .register_event_handler(GroupEventTypeKind::GroupAtMessageCreate, Self::handle_group);
    }
}
