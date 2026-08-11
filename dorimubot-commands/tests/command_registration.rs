use dorimubot_commands::{
    CommandHandler, CommandPlugin, CommandsStore, DynCommandHandleFn, ReplyingMessage,
};
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::common::{C2cUser, User};
use std::collections::HashMap;

fn command_message(content: &str) -> C2cMessage {
    C2cMessage {
        id: "message-id".to_string(),
        author: C2cUser {
            user: User {
                id: None,
                username: "test-user".to_string(),
                bot: false,
                union_openid: None,
                union_user_account: None,
            },
            user_openid: "user-id".to_string(),
        },
        content: Some(content.to_string()),
        timestamp: None,
        message_type: None,
        message_scene: None,
        attachments: None,
        ark_data: None,
        msg_elements: None,
    }
}

fn into_command_handler<H, Args, Kind>(handler: H) -> DynCommandHandleFn
where
    H: CommandHandler<Args, Kind>,
{
    handler.into_dyn()
}

async fn assert_text_response(handler: DynCommandHandleFn, message: &C2cMessage, expected: &str) {
    let response = match handler(message).await {
        Ok(response) => response,
        Err(error) => panic!("registered command handler failed: {error}"),
    };

    match response {
        Some(ReplyingMessage::Text(text)) => assert_eq!(text, expected),
        _ => panic!("registered command returned an unexpected response"),
    }
}

fn manually_registered_command() -> ReplyingMessage {
    ReplyingMessage::Text("registered manually".to_string())
}

struct MutableCommand {
    calls: usize,
}

impl MutableCommand {
    fn handle(&mut self) -> ReplyingMessage {
        self.calls += 1;
        ReplyingMessage::Text(format!("called {} times", self.calls))
    }
}

#[tokio::test]
async fn function_handler_is_registered_and_runs() {
    let mut commands = HashMap::new();
    commands.insert(
        "/manual-registration",
        into_command_handler(manually_registered_command),
    );
    let commands = CommandsStore::new(commands);

    let handler = commands
        .get("/manual-registration")
        .expect("function command should be registered");
    assert!(commands.get("/not-registered").is_none());
    assert_text_response(
        handler,
        &command_message("/manual-registration"),
        "registered manually",
    )
    .await;
}

#[tokio::test]
async fn stateful_closure_handler_is_registered_and_runs() {
    let location = "Earth".to_string();
    let mut commands = HashMap::new();
    commands.insert(
        "/closure-registration",
        into_command_handler(move || ReplyingMessage::Text(format!("Hi from {location}"))),
    );
    let commands = CommandsStore::new(commands);

    let handler = commands
        .get("/closure-registration")
        .expect("closure command should be registered");
    assert_text_response(
        handler,
        &command_message("/closure-registration"),
        "Hi from Earth",
    )
    .await;
}

#[tokio::test]
async fn mutable_method_handler_is_registered_and_runs_repeatedly() {
    let mut command = MutableCommand { calls: 0 };
    let handler = into_command_handler(move || command.handle());
    let message = command_message("/mutable-method");

    assert_text_response(handler.clone(), &message, "called 1 times").await;
    assert_text_response(handler, &message, "called 2 times").await;
}

#[test]
fn with_command_accepts_a_mutable_method_handler() {
    let mut command = MutableCommand { calls: 0 };

    let _plugin = CommandPlugin::new().with_command("/mutable-method", move || command.handle());
}
