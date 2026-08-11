mod common;

use dorimubot_framework_core::QQBot;
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

async fn group_app(register_handler: bool) -> QQBot {
    let app = QQBot::new(common::qqbot_config()).await;
    if register_handler {
        app.register_event_handler(GroupEventKind::GroupAtMessageCreate, group_message_handler);
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
async fn named_group_message_handler_is_scoped_to_the_registered_app() {
    let registered_app = group_app(true).await;
    let unregistered_app = group_app(false).await;

    GROUP_HANDLER_CALLS.store(0, Ordering::SeqCst);
    unregistered_app
        .webhook_handler(WebhookPayload::Dispatch(group_payload()))
        .await;
    registered_app
        .webhook_handler(WebhookPayload::Dispatch(group_payload()))
        .await;

    assert_eq!(GROUP_HANDLER_CALLS.load(Ordering::SeqCst), 1);
}

async fn app(register_handler: bool) -> (QQBot, Arc<HandlerState>) {
    let state = Arc::new(HandlerState {
        called: AtomicUsize::new(0),
    });
    let app = QQBot::new(common::qqbot_config()).await;
    if register_handler {
        let handler_state = Arc::clone(&state);
        app.register_event_handler(
            C2cEventKind::C2cMessageCreate,
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
    let (registered_app, registered_state) = app(true).await;
    let (unregistered_app, unregistered_state) = app(false).await;

    unregistered_app
        .webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;
    registered_app
        .webhook_handler(WebhookPayload::Dispatch(c2c_payload()))
        .await;

    assert_eq!(unregistered_state.called.load(Ordering::SeqCst), 0);
    assert_eq!(registered_state.called.load(Ordering::SeqCst), 1);
}
