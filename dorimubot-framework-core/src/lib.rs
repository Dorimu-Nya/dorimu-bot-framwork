//! QQ bot SDK (webhook-first).

mod app;
mod config;
mod dispatching;
mod event_handler;
mod event_registry;
mod registering;

pub use app::{QQApiCLient, QQBot};
pub use config::{CredentialConfig, ListeningConfig, QQBotConfig};
pub use event_handler::EventHandler;
pub use qqbot_rust_sdk;