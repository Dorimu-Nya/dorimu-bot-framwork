use crate::app::QQBot;
use crate::{EventHandler, TypedEventKind};
use qqbot_rust_sdk::events::payload::event::EventKind;

impl QQBot {
    /// 注册一个事件处理器。
    ///
    /// 事件原始 data 无法转换为 handler 参数时，注册会编译失败：
    ///
    /// ```compile_fail
    /// use dorimubot_framework_core::{QQBot, QQBotConfig, TypedEventKind};
    /// use qqbot_rust_sdk::events::c2c::models::C2cMessage;
    /// use qqbot_rust_sdk::events::group::event::GroupEventKind;
    /// use qqbot_rust_sdk::events::group::models::GroupMessage;
    ///
    /// let app = QQBot::new(QQBotConfig::new());
    /// let event = TypedEventKind::<_, GroupMessage>::new(
    ///     GroupEventKind::GroupMessageCreate,
    /// );
    /// app.register_event_handler(event, |_message: C2cMessage| {});
    /// ```
    pub fn register_event_handler<K, Data, Args, Kind, H>(
        &self,
        event: TypedEventKind<K, Data>,
        handler: H,
    ) where
        K: Into<EventKind>,
        H: EventHandler<Data, Args, Kind>,
    {
        self.event_handlers.register(event, handler);
    }
}
