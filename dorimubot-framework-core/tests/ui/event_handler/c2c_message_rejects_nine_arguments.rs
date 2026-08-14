use dorimubot_framework_core::{events::c2c, QQBot, QQBotConfig};
use qqbot_rust_sdk::events::c2c::models::C2cMessage;

fn main() {
    let app = QQBot::new(QQBotConfig::default());
    app.register_event_handler(
        c2c::C2cMessageCreate,
        |_a1: C2cMessage,
         _a2: C2cMessage,
         _a3: C2cMessage,
         _a4: C2cMessage,
         _a5: C2cMessage,
         _a6: C2cMessage,
         _a7: C2cMessage,
         _a8: C2cMessage,
         _a9: C2cMessage| {},
    );
}
