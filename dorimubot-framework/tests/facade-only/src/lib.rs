use dorimubot_framework::dorimubot_commands::command;

#[command("/ping")]
fn ping() {}

#[test]
fn command_macro_works_with_only_the_framework_facade() {
    assert!(dorimubot_framework::dorimubot_commands::inventory::iter::<
        dorimubot_framework::dorimubot_commands::CommandDef,
    >
        .into_iter()
        .any(|command| command.prefix == "/ping"));
}
