use dorimubot_commands::{CommandDef, CommandPlugin, ReplyingMessage};
use dorimubot_commands_macros::command;
use dorimubot_framework_core::{QQBot, QQBotConfig};

#[command("/macro-registration")]
fn macro_registered_command() {}

#[test]
fn command_macro_registers_command_definition() {
    assert!(dorimubot_commands::inventory::iter::<CommandDef>
        .into_iter()
        .any(|command| command.prefix == "/macro-registration"));
}

#[test]
fn command_without_macro_can_be_registered() {
    let app = QQBot::new(QQBotConfig::new());
    let command_plugin = CommandPlugin::new().with_command("/manual-registration", || {
        ReplyingMessage::Text("registered manually".to_string())
    });

    command_plugin.register(&app);
}
