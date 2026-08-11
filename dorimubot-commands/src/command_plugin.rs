use crate::{
    wrap_command_handle_fn, CommandDef, CommandHandler, CommandsStore, DynCommandHandleFn,
    ReplyingMessage,
};
use dorimubot_framework_core::{QQApiCLient, QQBot};
use qqbot_rust_sdk::events::c2c::event::C2cEventKind;
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::group::event::{GroupEvent, GroupEventKind};
use qqbot_rust_sdk::events::group::models::GroupMessage;
use qqbot_rust_sdk::events::payload::event::Event;
use qqbot_rust_sdk::events::payload::payload::DispatchPayload;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info, warn};

/// 将命令表注册为消息事件处理器。
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

    /// 配置是否忽略重复命令检查。
    pub fn ignore_checking(mut self, ignore: bool) -> Self {
        self.ignore_checking = ignore;
        self
    }

    /// 手动注册一个命令处理器。
    ///
    /// 处理器可以是 `Fn`、`FnMut`，也可以是实现了 [`crate::Command`] 或
    /// [`crate::AsyncCommand`] 的结构体。
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

    /// 将命令处理器注册到应用。
    pub fn register(&self, app: &QQBot) {
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

        let commands = CommandsStore::new(commands);

        let api = app.get_api_client();
        let c2c_commands = commands.clone();
        app.register_event_handler(C2cEventKind::C2cMessageCreate, move |message| {
            let api = Arc::clone(&api);
            let commands = c2c_commands.clone();
            async move { Self::handle_c2c(message, api, commands).await }
        });

        let api = app.get_api_client();
        let group_message_commands = commands.clone();
        app.register_event_handler(GroupEventKind::GroupAtMessageCreate, move |message| {
            let api = Arc::clone(&api);
            let commands = group_message_commands.clone();
            async move { Self::handle_group(message, api, commands).await }
        });

        let api = app.get_api_client();
        let bot_mention = app
            .bot_info()
            .and_then(|bot_info| bot_info.union_openid.as_deref())
            .filter(|union_openid| !union_openid.is_empty())
            .map(|union_openid| format!("@<{union_openid}>"));
        // SDK 的 GroupMessage 提取器目前只覆盖 GroupAtMessageCreate，
        // 全量群消息需要从完整载荷中显式取出。
        app.register_event_handler(
            GroupEventKind::GroupMessageCreate,
            move |payload: DispatchPayload| {
                let api = Arc::clone(&api);
                let commands = commands.clone();
                let bot_mention = bot_mention.clone();
                async move {
                    let Event::GroupEvent(GroupEvent::GroupMessageCreate(message)) = payload.event
                    else {
                        return;
                    };
                    Self::handle_group_message_create(message, bot_mention, api, commands).await
                }
            },
        );
    }

    async fn handle_c2c(message: C2cMessage, api: Arc<QQApiCLient>, commands: CommandsStore) {
        if let Some(reply) = Self::handle_message(&message, &commands).await {
            let body = reply.to_request(Some(message.id.clone()), Some(1));
            let result = api
                .c2c_messages()
                .send_typed(&message.author.user_openid, &body)
                .await;
            info!("send c2c message result: {:?}", result);
        }
    }

    async fn handle_group(message: GroupMessage, api: Arc<QQApiCLient>, commands: CommandsStore) {
        if let Some(reply) = Self::handle_message(&message, &commands).await {
            let body = reply.to_request(Some(message.id.clone()), Some(1));
            let result = api
                .group_messages()
                .send_typed(&message.group_openid, &body)
                .await;
            info!("send group message result: {:?}", result);
        }
    }

    async fn handle_group_message_create(
        mut message: GroupMessage,
        bot_mention: Option<String>,
        api: Arc<QQApiCLient>,
        commands: CommandsStore,
    ) {
        let Some(bot_mention) = bot_mention else {
            return;
        };
        let Some(content) = message.content.as_deref() else {
            return;
        };
        let Some(content) = content.strip_prefix(&bot_mention) else {
            return;
        };

        message.content = Some(content.trim().to_string());
        Self::handle_group(message, api, commands).await;
    }

    async fn handle_message(
        message: &dyn crate::CommonMessage,
        commands: &CommandsStore,
    ) -> Option<ReplyingMessage> {
        let content = message.get_content().as_deref()?;
        let command = content.split_whitespace().next()?;
        let Some(handler) = commands.get(command) else {
            warn!("未知指令: {}", content);
            return None;
        };

        match handler(message).await {
            Ok(reply) => reply,
            Err(err) => {
                error!("处理指令{}出错: {}", content, err);
                None
            }
        }
    }
}
