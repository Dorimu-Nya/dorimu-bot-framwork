//! QQ bot SDK (webhook-first).

extern crate self as dorimubot_framework;

#[cfg(feature = "app")]
mod app;
#[cfg(feature = "app")]
mod config;
#[cfg(feature = "app")]
mod dispatching;
#[cfg(feature = "runtime")]
mod event_handler;
#[cfg(feature = "runtime")]
mod event_registry;
#[cfg(feature = "app")]
mod plugin;
#[cfg(feature = "app")]
mod registering;

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

#[cfg(feature = "macros")]
pub use dorimubot_commands_macros::command;
#[cfg(feature = "macros")]
#[doc(hidden)]
pub use inventory;
#[cfg(feature = "app")]
#[doc(hidden)]
pub use serde;
#[cfg(feature = "app")]
#[doc(hidden)]
pub use tracing;

#[cfg(feature = "app")]
pub use app::{QQApiCLient, QQBotApp};
#[cfg(feature = "app")]
pub use config::{AppConfig, CredentialConfig, ListeningConfig, QQApiOverrides, SandboxConfig};
#[cfg(feature = "runtime")]
pub use event_handler::{
    AsyncEventHandlerKind, BorrowedEventSyncHandlerKind, DynEventHandler, EventHandler,
    EventHandlerFuture, SyncEventHandlerKind,
};
#[cfg(feature = "runtime")]
pub use event_registry::EventHandlerRegistry;
#[cfg(feature = "app")]
pub use plugin::Plugin;
