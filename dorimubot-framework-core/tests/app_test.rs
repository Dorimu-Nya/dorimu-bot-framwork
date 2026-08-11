mod common;

use dorimubot_framework_core::QQBot;
use qqbot_rust_sdk::events::c2c::event::C2cEventKind;
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use std::sync::atomic::{AtomicUsize, Ordering};

static HANDLER_CALLS: AtomicUsize = AtomicUsize::new(0);

#[tokio::test]
async fn registers_event_handlers_with_supported_signatures() {
    let app = QQBot::new(common::qqbot_config()).await;

    let bot_info = app.bot_info().unwrap();
    assert_eq!(bot_info.id, "test-bot-id");
    assert_eq!(bot_info.username, "test-bot");

    app.register_event_handler(C2cEventKind::C2cMessageCreate, handler_without_arguments);
    app.register_event_handler(C2cEventKind::C2cMessageCreate, handler_with_payload);
    app.register_event_handler(C2cEventKind::C2cMessageCreate, handler_with_event_detail);

    HANDLER_CALLS.store(0, Ordering::SeqCst);
    app.webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;

    assert_eq!(HANDLER_CALLS.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn bot_info_failure_does_not_prevent_initialization() {
    let app = QQBot::new(common::qqbot_config_without_bot_info()).await;

    assert!(app.bot_info().is_none());
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
            "content": "app-test",
            "attachments": [],
            "msg_elements": []
        }
    }))
    .unwrap()
}

fn handler_without_arguments() {
    HANDLER_CALLS.fetch_add(1, Ordering::SeqCst);
}

fn handler_with_payload(_payload: &DispatchPayload) {
    HANDLER_CALLS.fetch_add(1, Ordering::SeqCst);
}

fn handler_with_event_detail(_detail: &C2cMessage) {
    HANDLER_CALLS.fetch_add(1, Ordering::SeqCst);
}
