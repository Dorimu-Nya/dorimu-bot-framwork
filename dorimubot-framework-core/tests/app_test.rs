use dorimubot_framework_core::{QQBot, QQBotConfig, TypedEventKind};
use qqbot_rust_sdk::events::c2c::event::C2cEventKind;
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use std::sync::atomic::{AtomicUsize, Ordering};

static HANDLER_CALLS: AtomicUsize = AtomicUsize::new(0);

#[tokio::test]
async fn registers_event_handlers_with_supported_signatures() {
    let app = QQBot::new(QQBotConfig::default());

    let event = TypedEventKind::<_, C2cMessage>::new(C2cEventKind::C2cMessageCreate);
    app.register_event_handler(event, handler_without_arguments);
    app.register_event_handler(event, handler_with_event_detail);

    HANDLER_CALLS.store(0, Ordering::SeqCst);
    app.webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;

    assert_eq!(HANDLER_CALLS.load(Ordering::SeqCst), 2);
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

fn handler_with_event_detail(_detail: &C2cMessage) {
    HANDLER_CALLS.fetch_add(1, Ordering::SeqCst);
}
