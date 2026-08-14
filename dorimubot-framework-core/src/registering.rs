use crate::app::QQBot;
use crate::events::EventSpec;
use crate::EventHandler;

impl QQBot {
    /// 注册事件处理器。
    ///
    /// `event` 决定处理器可以接收的载荷类型。处理器的每个按值
    /// 参数都必须能由该载荷通过 [`Into`] 转换得到，最多支持八个参数；
    /// 同时也允许零参数处理器。
    pub fn register_event_handler<E, H, Args, Mode>(&self, event: E, handler: H)
    where
        E: EventSpec,
        H: EventHandler<E, Args, Mode>,
    {
        self.event_handlers.register(event, handler);
    }
}
