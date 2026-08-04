use qqbot_rust_sdk::events::c2c::event::C2cEventKind;
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use qqbot_sdk_app::{AppConfig, QQBotApp};
use qqbot_sdk_runtime::{Depend, Plugin, PluginRegistrar};
use std::sync::atomic::{AtomicUsize, Ordering};

struct PluginState {
    called: AtomicUsize,
}

struct TemporaryPlugin;

impl Plugin for TemporaryPlugin {
    fn register(&self, registrar: &PluginRegistrar<'_>) {
        registrar.insert_dependency(PluginState {
            called: AtomicUsize::new(0),
        });
        registrar.register_event_handler(
            C2cEventKind::C2cMessageCreate,
            |message: C2cMessage, state: Depend<PluginState>| async move {
                assert_eq!(message.content.as_deref(), Some("plugin-event"));
                state.called.fetch_add(1, Ordering::SeqCst);
            },
        );
    }
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
            "content": "plugin-event",
            "attachments": [],
            "msg_elements": []
        }
    }))
    .unwrap()
}

#[tokio::test]
async fn loaded_plugin_receives_dependencies_when_handling_events() {
    let app = QQBotApp::new(AppConfig::new().with_plugin(TemporaryPlugin));
    app.webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;

    assert_eq!(
        app.depend_store
            .get::<PluginState>()
            .called
            .load(Ordering::SeqCst),
        1
    );
}
