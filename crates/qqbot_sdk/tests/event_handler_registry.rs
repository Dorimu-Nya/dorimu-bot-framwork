use qqbot_sdk::events::c2c::event_type::C2cEventTypeKind;
use qqbot_sdk::events::c2c::models::C2cMessage;
use qqbot_sdk::events::payload::{DispatchPayload, WebhookPayload};
use qqbot_sdk::{App, AppConfig, Depend, Plugin, PluginRegistrar};
use std::sync::atomic::{AtomicUsize, Ordering};

struct HandlerState {
    called: AtomicUsize,
}

struct EventPlugin;

impl Plugin for EventPlugin {
    fn register(&self, registrar: &PluginRegistrar<'_>) {
        registrar.register_event_handler(
            C2cEventTypeKind::C2cMessageCreate,
            |_message: C2cMessage, state: Depend<HandlerState>| async move {
                state.called.fetch_add(1, Ordering::SeqCst);
            },
        );
    }
}

fn app() -> App {
    App::new(AppConfig::new().with_depend(Depend::new(HandlerState {
        called: AtomicUsize::new(0),
    })))
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
            "content": "event-registry",
            "msg_seq": 1
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
