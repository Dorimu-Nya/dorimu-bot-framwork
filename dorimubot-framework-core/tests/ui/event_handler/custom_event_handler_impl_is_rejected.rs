use dorimubot_framework_core::{events::c2c, EventHandler};
use qqbot_rust_sdk::events::payload::payload::DispatchPayload;

struct CustomHandler;
struct CustomMode;

impl EventHandler<c2c::C2cMessageCreate, (DispatchPayload,), CustomMode> for CustomHandler {}

fn main() {}
