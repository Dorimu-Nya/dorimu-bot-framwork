use dorimubot_framework::{AppConfig, Plugin, QQBotApp};
use qqbot_rust_sdk::events::c2c::event::C2cEventKind;
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct HandlerState {
    called: AtomicUsize,
}
struct EventPlugin {
    state: Arc<HandlerState>,
}

impl Plugin for EventPlugin {
    fn register(&self, app: &QQBotApp) {
        let state = Arc::clone(&self.state);
        app.registe_event_handler(
            C2cEventKind::C2cMessageCreate,
            move |_message: C2cMessage| {
                let state = Arc::clone(&state);
                async move {
                    state.called.fetch_add(1, Ordering::SeqCst);
                }
            },
        );
    }
}

fn app() -> (QQBotApp, Arc<HandlerState>) {
    let state = Arc::new(HandlerState {
        called: AtomicUsize::new(0),
    });
    (QQBotApp::new(AppConfig::new()), state)
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
            "content": "event-registry",
            "attachments": [],
            "msg_elements": []
        }
    }))
    .unwrap()
}

#[tokio::test]
async fn event_handlers_are_scoped_to_the_registered_app() {
    let (registered_app, registered_state) = app();
    let (unregistered_app, unregistered_state) = app();
    registered_app.registe_plugin(&EventPlugin {
        state: Arc::clone(&registered_state),
    });

    unregistered_app
        .webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;
    registered_app
        .webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;

    assert_eq!(unregistered_state.called.load(Ordering::SeqCst), 0);
    assert_eq!(registered_state.called.load(Ordering::SeqCst), 1);
}
