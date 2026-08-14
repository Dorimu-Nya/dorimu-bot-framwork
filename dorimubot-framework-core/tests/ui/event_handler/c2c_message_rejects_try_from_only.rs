use dorimubot_framework_core::{events::c2c, QQBot, QQBotConfig};
use qqbot_rust_sdk::events::c2c::models::C2cMessage;
use std::convert::Infallible;

struct TryFromOnly;

impl TryFrom<C2cMessage> for TryFromOnly {
    type Error = Infallible;

    fn try_from(_: C2cMessage) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

fn main() {
    let app = QQBot::new(QQBotConfig::default());
    app.register_event_handler(c2c::C2cMessageCreate, |_value: TryFromOnly| {});
}
