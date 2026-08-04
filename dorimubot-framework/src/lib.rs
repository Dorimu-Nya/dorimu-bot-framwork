//! All-in-one facade for DorimuBot Framework.

#[cfg(feature = "axum-webhook")]
mod runner;

pub use dorimubot_framework_core;

#[cfg(feature = "axum-webhook")]
pub use runner::run_dorimubot;

#[cfg(feature = "axum-webhook")]
pub use dorimubot_axum;

#[cfg(feature = "commands")]
pub use dorimubot_commands;

#[cfg(feature = "commands-macros")]
pub use dorimubot_commands_macros;
