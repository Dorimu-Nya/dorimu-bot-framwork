//! 以 Webhook 为主要接入方式的 QQ 机器人开发框架核心库。

mod app;
mod config;
mod dispatching;
mod event_handler;
mod event_registry;
pub mod events;
mod registering;

pub use app::{QQApiCLient, QQBot};
pub use config::{CredentialConfig, ListeningConfig, QQBotConfig};
pub use event_handler::EventHandler;
pub use qqbot_rust_sdk;
