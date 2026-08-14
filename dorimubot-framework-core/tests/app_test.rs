use dorimubot_framework_core::{events::c2c, QQBot, QQBotConfig};
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
struct MessageSummary {
    id: String,
    content: Option<String>,
}

impl From<C2cMessage> for MessageSummary {
    fn from(message: C2cMessage) -> Self {
        Self {
            id: message.id,
            content: message.content,
        }
    }
}

#[tokio::test]
async fn owned_handlers_receive_zero_one_two_and_async_arguments() {
    let app = QQBot::new(QQBotConfig::default());

    let zero_calls = Arc::new(AtomicUsize::new(0));
    let zero_handler_calls = Arc::clone(&zero_calls);
    app.register_event_handler(c2c::C2cMessageCreate, move || {
        zero_handler_calls.fetch_add(1, Ordering::SeqCst);
    });

    let one_calls = Arc::new(AtomicUsize::new(0));
    let one_handler_calls = Arc::clone(&one_calls);
    app.register_event_handler(c2c::C2cMessageCreate, move |message: C2cMessage| {
        assert_eq!(message.id, "message-id");
        assert_eq!(message.content.as_deref(), Some("typed handlers"));
        one_handler_calls.fetch_add(1, Ordering::SeqCst);
    });

    let two_calls = Arc::new(AtomicUsize::new(0));
    let two_handler_calls = Arc::clone(&two_calls);
    app.register_event_handler(
        c2c::C2cMessageCreate,
        move |message: C2cMessage, summary: MessageSummary| {
            assert_eq!(&message.id, &summary.id);
            assert_eq!(&message.content, &summary.content);
            assert_eq!(
                summary,
                MessageSummary {
                    id: "message-id".to_owned(),
                    content: Some("typed handlers".to_owned()),
                }
            );
            two_handler_calls.fetch_add(1, Ordering::SeqCst);
        },
    );

    let async_calls = Arc::new(AtomicUsize::new(0));
    let async_handler_calls = Arc::clone(&async_calls);
    app.register_event_handler(c2c::C2cMessageCreate, move |summary: MessageSummary| {
        let async_handler_calls = Arc::clone(&async_handler_calls);
        async move {
            assert_eq!(summary.id, "message-id");
            assert_eq!(summary.content.as_deref(), Some("typed handlers"));
            async_handler_calls.fetch_add(1, Ordering::SeqCst);
        }
    });

    app.webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;

    assert_eq!(zero_calls.load(Ordering::SeqCst), 1);
    assert_eq!(one_calls.load(Ordering::SeqCst), 1);
    assert_eq!(two_calls.load(Ordering::SeqCst), 1);
    assert_eq!(async_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn eight_owned_arguments_are_supported() {
    let app = QQBot::new(QQBotConfig::default());
    let calls = Arc::new(AtomicUsize::new(0));
    let handler_calls = Arc::clone(&calls);

    app.register_event_handler(
        c2c::C2cMessageCreate,
        move |a1: C2cMessage,
              a2: C2cMessage,
              a3: C2cMessage,
              a4: C2cMessage,
              a5: C2cMessage,
              a6: C2cMessage,
              a7: C2cMessage,
              a8: C2cMessage| {
            for message in [&a1, &a2, &a3, &a4, &a5, &a6, &a7, &a8] {
                assert_eq!(message.id, "message-id");
                assert_eq!(message.content.as_deref(), Some("typed handlers"));
            }
            handler_calls.fetch_add(1, Ordering::SeqCst);
        },
    );

    app.webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;

    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

fn c2c_payload() -> DispatchPayload {
    serde_json::from_value(serde_json::json!({
        "id": "event-id", "op": 0, "s": 1, "t": "C2C_MESSAGE_CREATE",
        "d": {
            "id": "message-id",
            "author": {
                "username": "",
                "bot": false,
                "union_user_account": "",
                "user_openid": "user-id",
                "member_open_id": "",
                "membership_role": ""
            },
            "content": "typed handlers",
            "attachments": [],
            "msg_elements": []
        }
    }))
    .unwrap()
}
