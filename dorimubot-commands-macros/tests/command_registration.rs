use dorimubot_commands::CommandDef;
use dorimubot_commands_macros::command;

#[command("/macro-registration")]
fn macro_registered_command() {}

#[test]
fn command_macro_registers_command_definition() {
    assert!(dorimubot_commands::inventory::iter::<CommandDef>
        .into_iter()
        .any(|command| command.prefix == "/macro-registration"));
}
