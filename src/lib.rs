//! QQ bot SDK (webhook-first).

extern crate self as qqbot_sdk;

#[allow(unused_imports)]
pub use qqbot_sdk_core::*;

#[cfg(feature = "runtime")]
pub use qqbot_sdk_runtime::*;

#[cfg(feature = "commands")]
pub use qqbot_sdk_commands::{
    CommandDef, CommandHandleFn, CommandHandleFuture, CommandHandler, CommandOutput, CommonMessage,
    DynCommandHandleFn, FromCommandArg, FromCommonMessage, MessageFrom, ReplyingMessage,
    ReplyingType,
};

#[cfg(feature = "command-plugin")]
pub use qqbot_sdk_commands::CommandPlugin;

#[cfg(feature = "macros")]
pub use qqbot_sdk_commands::command;

#[cfg(feature = "app")]
pub use qqbot_sdk_app::{
    AppConfig, CredentialConfig, ListeningConfig, QQApiOverrides, QQBotApp, SandboxConfig,
};

#[cfg(feature = "macros")]
pub use inventory;

#[cfg(feature = "axum-runner")]
pub use qqbot_sdk_axum::{run_application, run_application_with_router};
