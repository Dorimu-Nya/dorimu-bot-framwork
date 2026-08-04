extern crate self as dorimubot_framework;

pub use dorimubot_commands::{CommandDef, CommandHandleFuture, CommandOutput, CommonMessage};
pub use dorimubot_runtime::DependencyProvider;
pub use inventory;

use dorimubot_commands_macros::command;

#[command("/macro-registration")]
fn macro_registered_command() {}

#[test]
fn command_macro_registers_command_definition() {
    assert!(inventory::iter::<CommandDef>
        .into_iter()
        .any(|command| command.prefix == "/macro-registration"));
}
