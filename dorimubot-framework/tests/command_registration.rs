mod common;

use dorimubot_commands::{CommandDef, CommandPlugin, ReplyingMessage};
use dorimubot_commands_macros::command;
use dorimubot_framework_core::QQBot;
use qqbot_rust_sdk::events::common::{GroupUser, MessageScene, User};
use qqbot_rust_sdk::events::group::event::GroupEvent;
use qqbot_rust_sdk::events::group::models::GroupMessage;
use qqbot_rust_sdk::events::payload::event::Event;
use qqbot_rust_sdk::events::payload::opcode::DispatchOp;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use std::sync::{Arc, Mutex};

#[command("/macro-registration")]
fn macro_registered_command() {}

#[test]
fn command_macro_registers_command_definition() {
    assert!(dorimubot_commands::inventory::iter::<CommandDef>
        .into_iter()
        .any(|command| command.prefix == "/macro-registration"));
}

#[tokio::test]
async fn command_without_macro_can_be_registered() {
    let app = QQBot::new(common::qqbot_config()).await;
    let command_plugin = CommandPlugin::new().with_command("/manual-registration", || {
        ReplyingMessage::Text("registered manually".to_string())
    });

    command_plugin.register(&app);
}

fn group_message_payload(content: &str) -> DispatchPayload {
    DispatchPayload {
        id: Some("event-id".to_string()),
        s: Some(1),
        op: DispatchOp::Dispatch,
        event: Event::GroupEvent(GroupEvent::GroupMessageCreate(GroupMessage {
            id: "message-id".to_string(),
            author: GroupUser {
                user: User {
                    id: None,
                    username: "test-user".to_string(),
                    bot: false,
                    union_openid: None,
                    union_user_account: None,
                },
                member_openid: "member-id".to_string(),
                member_role: "member".to_string(),
            },
            content: Some(content.to_string()),
            group_openid: "group-id".to_string(),
            timestamp: None,
            message_type: 0,
            message_scene: MessageScene {
                source: None,
                ext: None,
            },
            attachments: None,
            mentions: None,
            ark_data: None,
            msg_elements: None,
        })),
    }
}

#[tokio::test]
async fn group_message_command_requires_bot_mention_and_trims_content() {
    let app = QQBot::new(common::qqbot_config()).await;
    let handled_contents = Arc::new(Mutex::new(Vec::new()));
    let handler_contents = Arc::clone(&handled_contents);
    let command_plugin =
        CommandPlugin::new().with_command("/group-message", move |content: String| {
            handler_contents.lock().unwrap().push(content);
        });
    command_plugin.register(&app);

    app.webhook_handler(WebhookPayload::Dispatch(group_message_payload(
        "@<test-bot-union-openid>   /group-message argument   ",
    )))
    .await;
    app.webhook_handler(WebhookPayload::Dispatch(group_message_payload(
        "/group-message without-mention",
    )))
    .await;
    app.webhook_handler(WebhookPayload::Dispatch(group_message_payload(
        "@another-bot /group-message wrong-mention",
    )))
    .await;

    assert_eq!(
        *handled_contents.lock().unwrap(),
        vec!["/group-message argument"]
    );
}
