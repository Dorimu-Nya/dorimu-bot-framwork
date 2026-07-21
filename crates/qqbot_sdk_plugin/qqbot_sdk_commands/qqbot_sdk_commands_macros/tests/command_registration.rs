extern crate self as qqbot_sdk;

pub use inventory;
pub use qqbot_sdk_commands::{CommandDef, CommandHandleFuture, CommandOutput, CommonMessage};
pub use qqbot_sdk_runtime::DependencyProvider;

use qqbot_sdk_commands_macros::command;

#[command("/macro-registration")]
fn macro_registered_command() {}

#[test]
fn command_macro_registers_command_definition() {
    assert!(inventory::iter::<CommandDef>
        .into_iter()
        .any(|command| command.prefix == "/macro-registration"));
}
