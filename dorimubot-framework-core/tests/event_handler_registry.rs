use dorimubot_framework_core::{QQBot, QQBotConfig, TypedEventKind};
use qqbot_rust_sdk::events::c2c::event::C2cEventKind;
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::group::event::GroupEventKind;
use qqbot_rust_sdk::events::group::models::GroupMessage;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct HandlerState {
    called: AtomicUsize,
}

static GROUP_HANDLER_CALLS: AtomicUsize = AtomicUsize::new(0);

fn group_message_handler(_message: GroupMessage) {
    GROUP_HANDLER_CALLS.fetch_add(1, Ordering::SeqCst);
}

fn group_app(register_handler: bool) -> QQBot {
    let app = QQBot::new(QQBotConfig::new());
    if register_handler {
        app.register_event_handler(
            TypedEventKind::<_, GroupMessage>::new(GroupEventKind::GroupAtMessageCreate),
            group_message_handler,
        );
    }
    app
}

fn group_payload() -> DispatchPayload {
    serde_json::from_value(serde_json::json!({
        "id": "event-id", "op": 0, "s": 1, "t": "GROUP_AT_MESSAGE_CREATE",
        "d": {
            "id": "message-id",
            "author": {
                "username": "",
                "bot": false,
                "union_user_account": "",
                "member_openid": "member-id",
                "member_role": "member"
            },
            "content": "event-registry",
            "group_openid": "group-id",
            "message_type": 0,
            "message_scene": { "ext": [] },
            "attachments": [],
            "mentions": [],
            "ark_data": { "prompt": "", "ark_type": "", "ark_name": "", "fields": {} },
            "msg_elements": []
        }
    }))
    .unwrap()
}

#[tokio::test]
async fn group_message_create_passes_variant_data() {
    static CALLS: AtomicUsize = AtomicUsize::new(0);

    struct MessageContent(Option<String>);

    impl From<GroupMessage> for MessageContent {
        fn from(message: GroupMessage) -> Self {
            Self(message.content)
        }
    }

    fn borrowed_handler(message: &GroupMessage) {
        assert_eq!(message.content.as_deref(), Some("event-registry"));
        CALLS.fetch_add(1, Ordering::SeqCst);
    }

    let app = QQBot::new(QQBotConfig::new());
    let event = TypedEventKind::<_, GroupMessage>::new(GroupEventKind::GroupMessageCreate);
    app.register_event_handler(event, borrowed_handler);
    app.register_event_handler(event, |message: GroupMessage| async move {
        assert_eq!(message.content.as_deref(), Some("event-registry"));
        CALLS.fetch_add(1, Ordering::SeqCst);
    });
    app.register_event_handler(event, |content: MessageContent| {
        assert_eq!(content.0.as_deref(), Some("event-registry"));
        CALLS.fetch_add(1, Ordering::SeqCst);
    });
    CALLS.store(0, Ordering::SeqCst);

    let mut payload = group_payload();
    payload.event = serde_json::from_value(serde_json::json!({
        "t": "GROUP_MESSAGE_CREATE",
        "d": serde_json::to_value(match &payload.event {
            qqbot_rust_sdk::events::payload::event::Event::GroupEvent(
                qqbot_rust_sdk::events::group::event::GroupEvent::GroupAtMessageCreate(message),
            ) => message,
            _ => unreachable!(),
        }).unwrap()
    }))
    .unwrap();

    app.webhook_handler(WebhookPayload::Dispatch(payload)).await;
    assert_eq!(CALLS.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn named_group_message_handler_is_scoped_to_the_registered_app() {
    let registered_app = group_app(true);
    let unregistered_app = group_app(false);

    GROUP_HANDLER_CALLS.store(0, Ordering::SeqCst);
    unregistered_app
        .webhook_handler(WebhookPayload::Dispatch(group_payload()))
        .await;
    registered_app
        .webhook_handler(WebhookPayload::Dispatch(group_payload()))
        .await;

    assert_eq!(GROUP_HANDLER_CALLS.load(Ordering::SeqCst), 1);
}

fn app(register_handler: bool) -> (QQBot, Arc<HandlerState>) {
    let state = Arc::new(HandlerState {
        called: AtomicUsize::new(0),
    });
    let app = QQBot::new(QQBotConfig::new());
    if register_handler {
        let handler_state = Arc::clone(&state);
        app.register_event_handler(
            TypedEventKind::<_, C2cMessage>::new(C2cEventKind::C2cMessageCreate),
            move |_message: C2cMessage| {
                let state = Arc::clone(&handler_state);
                async move {
                    state.called.fetch_add(1, Ordering::SeqCst);
                }
            },
        );
    }
    (app, state)
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
    let (registered_app, registered_state) = app(true);
    let (unregistered_app, unregistered_state) = app(false);

    unregistered_app
        .webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;
    registered_app
        .webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;

    assert_eq!(unregistered_state.called.load(Ordering::SeqCst), 0);
    assert_eq!(registered_state.called.load(Ordering::SeqCst), 1);
}
