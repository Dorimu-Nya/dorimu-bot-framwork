//! QQ bot SDK (webhook-first).

extern crate self as qqbot_sdk;

pub use qqbot_sdk_core::*;

#[cfg(feature = "commands")]
pub use qqbot_sdk_commands::{
    CommandDef, CommandHandleFn, CommandHandleFuture, CommandHandler, CommandOutput,
    DynCommandHandleFn, FromCommandArg, ReplyingMessage, ReplyingType,
};

#[cfg(feature = "macros")]
pub use qqbot_sdk_commands::command;

#[cfg(feature = "app")]
pub use qqbot_sdk_app::{
    App, AppConfig, AsyncEventHandlerKind, BorrowedEventSyncHandlerKind, CredentialConfig, Depend,
    DependStore, DynEventHandler, EventHandler, EventHandlerFuture, EventKind, ListeningConfig,
    Plugin, SandboxConfig, SyncEventHandlerKind,
};

#[cfg(feature = "macros")]
pub use inventory;

#[cfg(feature = "axum-runner")]
pub use qqbot_sdk_axum::{run_application, run_application_with_router};
