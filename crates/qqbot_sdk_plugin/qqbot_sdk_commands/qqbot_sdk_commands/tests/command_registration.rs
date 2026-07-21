use qqbot_sdk_commands::{CommandHandler, CommandsStore, DynCommandHandleFn, ReplyingMessage};
use qqbot_sdk_core::events::c2c::models::{C2cAuthor, C2cMessage};
use qqbot_sdk_runtime::DependStore;
use std::collections::HashMap;

fn command_message(content: &str) -> C2cMessage {
    C2cMessage {
        id: "message-id".to_string(),
        author: C2cAuthor {
            id: None,
            user_openid: "user-id".to_string(),
            union_openid: None,
        },
        content: Some(content.to_string()),
        timestamp: None,
        attachments: None,
        message_type: None,
        message_scene: None,
        msg_seq: Some(1),
    }
}

fn into_command_handler<H, Args, Kind>(handler: H) -> DynCommandHandleFn
where
    H: CommandHandler<Args, Kind>,
{
    handler.into_dyn()
}

async fn assert_text_response(handler: DynCommandHandleFn, message: &C2cMessage, expected: &str) {
    let dependencies = DependStore::new();
    let response = match handler(message, &dependencies).await {
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
