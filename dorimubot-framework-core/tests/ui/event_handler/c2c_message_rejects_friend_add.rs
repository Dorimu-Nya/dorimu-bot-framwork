use dorimubot_framework_core::{events::c2c, QQBot, QQBotConfig};
use qqbot_rust_sdk::events::c2c::models::FriendAddEvent;

fn main() {
    let app = QQBot::new(QQBotConfig::default());
    app.register_event_handler(c2c::C2cMessageCreate, |_event: FriendAddEvent| {});
}
