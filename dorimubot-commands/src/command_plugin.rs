use crate::{
    wrap_command_handle_fn, CommandDef, CommandHandler, CommandsStore, DynCommandHandleFn,
    ReplyingMessage,
};
use dorimubot_framework_core::{events, QQBot};
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::group::models::{GroupMention, GroupMessage};
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

        let bot = app.clone();
        let c2c_commands = commands.clone();
        app.register_event_handler(events::c2c::C2cMessageCreate, move |message: C2cMessage| {
            let bot = bot.clone();
            let commands = c2c_commands.clone();
            async move { Self::handle_c2c(message, bot, commands).await }
        });

        let bot = app.clone();
        let group_message_commands = commands.clone();
        app.register_event_handler(
            events::group::GroupAtMessageCreate,
            move |message: GroupMessage| {
                let bot = bot.clone();
                let commands = group_message_commands.clone();
                async move { Self::handle_group(message, bot, commands).await }
            },
        );

        let bot = app.clone();
        app.register_event_handler(
            events::group::GroupMessageCreate,
            move |message: GroupMessage| {
                let bot = bot.clone();
                let commands = commands.clone();
                async move { Self::handle_group_message_create(message, bot, commands).await }
            },
        );
    }

    async fn handle_c2c(message: C2cMessage, bot: QQBot, commands: CommandsStore) {
        if let Some(reply) = Self::handle_message(&message, &commands).await {
            let body = reply.to_request(Some(message.id.clone()), Some(1));
            let result = match bot.get_api_client().await {
                Ok(api) => {
                    api.message()
                        .c2c()
                        .post_c2c_message(&message.author.user_openid, &body)
                        .await
                }
                Err(error) => {
                    error!("initializing QQ API client failed: {error}");
                    return;
                }
            };
            info!("send c2c message result: {:?}", result);
        }
    }

    async fn handle_group(message: GroupMessage, bot: QQBot, commands: CommandsStore) {
        if let Some(reply) = Self::handle_message(&message, &commands).await {
            let body = reply.to_request(Some(message.id.clone()), Some(1));
            let result = match bot.get_api_client().await {
                Ok(api) => {
                    api.message()
                        .group()
                        .post_group_message(&message.group_openid, &body)
                        .await
                }
                Err(error) => {
                    error!("initializing QQ API client failed: {error}");
                    return;
                }
            };
            info!("send group message result: {:?}", result);
        }
    }

    async fn handle_group_message_create(
        message: GroupMessage,
        bot: QQBot,
        commands: CommandsStore,
    ) {
        if let Some(mentions) = &message.mentions {
            if mentions
                .iter()
                .any(|m| matches!(m, GroupMention::Single(m) if m.is_you))
            {
                let mut message = message.clone();
                if let Some(content) = &message.content {
                    message.content = Some(
                        regex::Regex::new(r"<@[A-Za-z0-9]+>")
                            .unwrap()
                            .replace_all(content.as_str(), "")
                            .trim()
                            .to_string(),
                    )
                }
                Self::handle_group(message, bot, commands).await;
            }
        }
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
