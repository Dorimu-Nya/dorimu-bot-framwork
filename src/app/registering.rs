use super::plugin::Plugin;
use crate::{app::App, EventHandler, KindRegistryKey};

impl App {
    /// 加载一个插件并让它注册所需的事件处理器。
    ///
    /// 参数使用 trait object，使原生插件与未来的 WASM 插件适配器能够共享此入口。
    pub fn registe_plugin(&self, plugin: &dyn Plugin) {
        plugin.register(self);
    }

    /// 注册一个事件处理器。
    pub fn registe_event_handler<K, Args, Kind, H>(&self, kind: K, handler: H)
    where
        K: KindRegistryKey,
        H: EventHandler<Args, Kind>,
    {
        kind.get_writable_vec().push(handler.into_dyn());
    }
}
