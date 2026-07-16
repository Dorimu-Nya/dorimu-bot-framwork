use crate::app::QQBotApp;
use qqbot_sdk_core::EventKind;
use qqbot_sdk_runtime::{EventHandler, Plugin, PluginRegistrar};

impl QQBotApp {
    /// 加载一个插件并让它注册所需的事件处理器。
    pub fn registe_plugin(&self, plugin: &dyn Plugin) {
        plugin.register(&PluginRegistrar::new(
            &self.event_handlers,
            &self.depend_store,
        ));
    }

    /// 注册一个事件处理器。
    pub fn registe_event_handler<K, Args, Kind, H>(&self, kind: K, handler: H)
    where
        K: Into<EventKind>,
        H: EventHandler<Args, Kind>,
    {
        self.event_handlers.register(kind, handler);
    }
}
