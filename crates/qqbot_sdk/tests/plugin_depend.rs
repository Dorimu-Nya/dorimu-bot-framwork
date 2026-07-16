use qqbot_sdk::events::c2c::event_type::C2cEventTypeKind;
use qqbot_sdk::events::c2c::models::C2cMessage;
use qqbot_sdk::events::payload::{DispatchPayload, WebhookPayload};
use qqbot_sdk::{App, AppConfig, Depend, Plugin, PluginRegistrar};
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
            C2cEventTypeKind::C2cMessageCreate,
            |message: C2cMessage, state: Depend<PluginState>| async move {
                assert_eq!(message.content.as_deref(), Some("plugin-event"));
                state.called.fetch_add(1, Ordering::SeqCst);
            },
        );
    }
}

fn c2c_payload() -> DispatchPayload {
    serde_json::from_value(serde_json::json!({
        "id": "event-id",
        "op": 0,
        "s": 1,
        "t": "C2C_MESSAGE_CREATE",
        "d": {
            "id": "message-id",
            "author": { "user_openid": "user-id" },
            "content": "plugin-event",
            "msg_seq": 1
        }
    }))
    .unwrap()
}

#[tokio::test]
async fn loaded_plugin_receives_dependencies_when_handling_events() {
    let app = App::new(AppConfig::new().with_plugin(TemporaryPlugin));

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
