//! All-in-one facade for DorimuBot Framework.

pub use dorimubot_axum as axum;
pub use dorimubot_commands as commands;
pub use dorimubot_commands_macros as commands_macros;
pub use dorimubot_framework_core as core;

pub use dorimubot_axum::{run_application, run_application_with_router};
pub use dorimubot_commands::*;
pub use dorimubot_framework_core::*;
