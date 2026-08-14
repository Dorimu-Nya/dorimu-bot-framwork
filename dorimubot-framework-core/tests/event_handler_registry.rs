use dorimubot_framework_core::{events::group, QQBot, QQBotConfig};
use qqbot_rust_sdk::events::group::event::GroupEvent;
use qqbot_rust_sdk::events::group::models::GroupMessage;
use qqbot_rust_sdk::events::payload::event::Event;
use qqbot_rust_sdk::events::payload::opcode::DispatchOp;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[tokio::test]
async fn markers_with_the_same_payload_are_dispatched_independently() {
    let app = QQBot::new(QQBotConfig::default());
    let at_calls = Arc::new(AtomicUsize::new(0));
    let all_calls = Arc::new(AtomicUsize::new(0));

    let at_handler_calls = Arc::clone(&at_calls);
    app.register_event_handler(group::GroupAtMessageCreate, move |message: GroupMessage| {
        assert_eq!(message.content.as_deref(), Some("marker isolation"));
        at_handler_calls.fetch_add(1, Ordering::SeqCst);
    });

    let all_handler_calls = Arc::clone(&all_calls);
    app.register_event_handler(group::GroupMessageCreate, move |message: GroupMessage| {
        assert_eq!(message.content.as_deref(), Some("marker isolation"));
        all_handler_calls.fetch_add(1, Ordering::SeqCst);
    });

    let message = group_message();
    dispatch(
        &app,
        Event::GroupEvent(GroupEvent::GroupAtMessageCreate(message.clone())),
    )
    .await;
    assert_eq!(at_calls.load(Ordering::SeqCst), 1);
    assert_eq!(all_calls.load(Ordering::SeqCst), 0);

    dispatch(
        &app,
        Event::GroupEvent(GroupEvent::GroupMessageCreate(message)),
    )
    .await;
    assert_eq!(at_calls.load(Ordering::SeqCst), 1);
    assert_eq!(all_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn async_handler_finishes_before_the_next_sync_handler_runs() {
    let app = QQBot::new(QQBotConfig::default());
    let stage = Arc::new(AtomicUsize::new(0));

    let async_stage = Arc::clone(&stage);
    app.register_event_handler(
        group::GroupAtMessageCreate,
        move |_message: GroupMessage| {
            let async_stage = Arc::clone(&async_stage);
            async move {
                assert_eq!(async_stage.swap(1, Ordering::SeqCst), 0);
                tokio::task::yield_now().await;
                assert_eq!(async_stage.swap(2, Ordering::SeqCst), 1);
            }
        },
    );

    let sync_stage = Arc::clone(&stage);
    app.register_event_handler(
        group::GroupAtMessageCreate,
        move |_message: GroupMessage| {
            assert_eq!(sync_stage.swap(3, Ordering::SeqCst), 2);
        },
    );

    dispatch(
        &app,
        Event::GroupEvent(GroupEvent::GroupAtMessageCreate(group_message())),
    )
    .await;

    assert_eq!(stage.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn unit_payload_marker_delivers_unit() {
    let app = QQBot::new(QQBotConfig::default());
    let calls = Arc::new(AtomicUsize::new(0));
    let handler_calls = Arc::clone(&calls);

    app.register_event_handler(group::SubscribeMessageStatus, move |payload: ()| {
        assert_eq!(payload, ());
        handler_calls.fetch_add(1, Ordering::SeqCst);
    });

    dispatch(&app, Event::GroupEvent(GroupEvent::SubscribeMessageStatus)).await;

    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn handlers_are_scoped_to_the_qqbot_that_registered_them() {
    let registered_app = QQBot::new(QQBotConfig::default());
    let other_app = QQBot::new(QQBotConfig::default());
    let calls = Arc::new(AtomicUsize::new(0));
    let handler_calls = Arc::clone(&calls);

    registered_app.register_event_handler(
        group::GroupAtMessageCreate,
        move |_message: GroupMessage| {
            handler_calls.fetch_add(1, Ordering::SeqCst);
        },
    );

    let message = group_message();
    dispatch(
        &other_app,
        Event::GroupEvent(GroupEvent::GroupAtMessageCreate(message.clone())),
    )
    .await;
    dispatch(
        &registered_app,
        Event::GroupEvent(GroupEvent::GroupAtMessageCreate(message)),
    )
    .await;

    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

async fn dispatch(app: &QQBot, event: Event) {
    app.webhook_handler(WebhookPayload::Dispatch(DispatchPayload {
        id: Some("event-id".to_owned()),
        s: Some(1),
        op: DispatchOp::Dispatch,
        event,
    }))
    .await;
}

fn group_message() -> GroupMessage {
    serde_json::from_value(serde_json::json!({
        "id": "message-id",
        "author": {
            "username": "",
            "bot": false,
            "union_user_account": "",
            "member_openid": "member-id",
            "member_role": "member"
        },
        "content": "marker isolation",
        "group_openid": "group-id",
        "message_type": 0,
        "message_scene": { "ext": [] },
        "attachments": [],
        "mentions": [],
        "ark_data": { "prompt": "", "ark_type": "", "ark_name": "", "fields": {} },
        "msg_elements": []
    }))
    .unwrap()
}
