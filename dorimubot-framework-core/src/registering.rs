use crate::app::QQBot;
use crate::EventHandler;
use qqbot_rust_sdk::events::payload::event::EventKind;

impl QQBot {
    /// 注册一个事件处理器。
    pub fn register_event_handler<K, Args, Kind, H>(&self, kind: K, handler: H)
    where
        K: Into<EventKind>,
        H: EventHandler<Args, Kind>,
    {
        self.event_handlers.register(kind, handler);
    }
}
