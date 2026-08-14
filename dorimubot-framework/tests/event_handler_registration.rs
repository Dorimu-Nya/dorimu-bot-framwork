use dorimubot_framework_core::{events, QQBot, QQBotConfig};
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use qqbot_rust_sdk::events::group::models::GroupMessage;

fn group_message_handler(message: GroupMessage) {
    println!("group_message_handler: {:?}", message);
}

#[test]
fn closure_event_handler_can_be_registered() {
    let app = QQBot::new(QQBotConfig::new());

    app.register_event_handler(events::c2c::C2cMessageCreate, move |message: C2cMessage| {
        println!("收到消息:{:?}", message);
    });
}

#[test]
fn function_event_handler_can_be_registered() {
    let app = QQBot::new(QQBotConfig::new());

    app.register_event_handler(events::group::GroupAtMessageCreate, group_message_handler);
}
