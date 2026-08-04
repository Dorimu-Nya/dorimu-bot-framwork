//! QQ bot SDK (webhook-first).

mod app;
mod config;
mod dispatching;
mod event_handler;
mod event_registry;
mod registering;

pub use app::{QQApiCLient, QQBot};
pub use config::{QQBotConfig, CredentialConfig, ListeningConfig};
pub use event_handler::{
    AsyncEventHandlerKind, BorrowedEventSyncHandlerKind, DynEventHandler, EventHandler,
    EventHandlerFuture, SyncEventHandlerKind,
};
