use qqbot_rust_sdk::events::c2c::event::C2cEventKind;
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use qqbot_sdk_app::{AppConfig, QQBotApp};
use qqbot_sdk_runtime::{Depend, Plugin, PluginRegistrar};
use std::sync::atomic::{AtomicUsize, Ordering};

struct HandlerState {
    called: AtomicUsize,
}
struct EventPlugin;

impl Plugin for EventPlugin {
    fn register(&self, registrar: &PluginRegistrar<'_>) {
        registrar.register_event_handler(
            C2cEventKind::C2cMessageCreate,
            |_message: C2cMessage, state: Depend<HandlerState>| async move {
                state.called.fetch_add(1, Ordering::SeqCst);
            },
        );
    }
}

fn app() -> QQBotApp {
    QQBotApp::new(AppConfig::new().with_depend(Depend::new(HandlerState {
        called: AtomicUsize::new(0),
    })))
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
    let registered_app = app();
    let unregistered_app = app();
    registered_app.registe_plugin(&EventPlugin);

    unregistered_app
        .webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;
    registered_app
        .webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;

    assert_eq!(
        unregistered_app
            .depend_store
            .get::<HandlerState>()
            .called
            .load(Ordering::SeqCst),
        0
    );
    assert_eq!(
        registered_app
            .depend_store
            .get::<HandlerState>()
            .called
            .load(Ordering::SeqCst),
        1
    );
}
