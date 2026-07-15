//! QQ bot SDK (webhook-first).

extern crate self as qqbot_sdk;

pub use qqbot_sdk_core::*;

#[cfg(feature = "commands")]
pub use qqbot_sdk_commands::{
    CommandDef, CommandHandleFn, CommandHandleFuture, CommandHandler, CommandOutput, Context,
    ContextStore, DynCommandHandleFn, FromCommandArg, ReplyingMessage, ReplyingType,
};

#[cfg(feature = "app")]
pub use qqbot_sdk_app::{
    App, AppConfig, AsyncEventHandlerKind, BorrowedEventSyncHandlerKind, CredentialConfig,
    DynEventHandler, EventHandler, EventHandlerFuture, KindRegistryKey, ListeningConfig, Plugin,
    SandboxConfig, SyncEventHandlerKind,
};

#[cfg(feature = "macros")]
pub use inventory;

#[cfg(feature = "axum-runner")]
pub use qqbot_sdk_axum::{run_application, run_application_with_router};
