//! QQ bot SDK (webhook-first).

extern crate self as dorimubot_framework;

#[allow(unused_imports)]
pub use qqbot_rust_sdk::*;

#[cfg(feature = "events")]
pub use qqbot_rust_sdk::events::payload::event::{Event, EventKind};
#[cfg(feature = "events")]
pub use qqbot_rust_sdk::events::payload::payload::{
    DispatchPayload, FromDispatchPayload, HttpCallbackAckPayload, WebhookAddressVerifyPayload,
    WebhookPayload,
};

#[cfg(feature = "openapi")]
pub use qqbot_rust_sdk::openapi::error::{Error, Result};
#[cfg(feature = "openapi")]
pub use qqbot_rust_sdk::openapi::http::{HttpClient, RetryPolicy};
#[cfg(feature = "openapi")]
pub use qqbot_rust_sdk::openapi::models;
#[cfg(feature = "openapi")]
pub use qqbot_rust_sdk::openapi::*;

#[cfg(feature = "signature")]
pub use qqbot_rust_sdk::signature::sign_webhook_validation;

#[cfg(feature = "runtime")]
pub use dorimubot_runtime::*;

#[cfg(feature = "commands")]
pub use dorimubot_commands::{
    CommandDef, CommandHandleFn, CommandHandleFuture, CommandHandler, CommandOutput, CommonMessage,
    DynCommandHandleFn, FromCommandArg, FromCommonMessage, MessageFrom, ReplyingMessage,
    ReplyingType,
};

#[cfg(feature = "command-plugin")]
pub use dorimubot_commands::CommandPlugin;

#[cfg(feature = "macros")]
pub use dorimubot_commands::command;

#[cfg(feature = "app")]
pub use dorimubot_app::{
    AppConfig, CredentialConfig, ListeningConfig, QQApiOverrides, QQBotApp, SandboxConfig,
};

#[cfg(feature = "macros")]
pub use inventory;

#[cfg(feature = "axum-runner")]
pub use dorimubot_axum::{run_application, run_application_with_router};
