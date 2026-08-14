use dorimubot_framework_core::{events::c2c, QQBot, QQBotConfig};
use qqbot_rust_sdk::events::c2c::models::C2cMessage;

struct ReverseFrom(C2cMessage);

impl From<ReverseFrom> for C2cMessage {
    fn from(value: ReverseFrom) -> Self {
        value.0
    }
}

fn main() {
    let app = QQBot::new(QQBotConfig::default());
    app.register_event_handler(c2c::C2cMessageCreate, |_value: ReverseFrom| {});
}
