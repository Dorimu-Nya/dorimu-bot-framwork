use crate::{DependStore, EventHandler, EventHandlerRegistry};
use qqbot_sdk_core::EventKind;

/// 插件注册期间可访问的运行时能力。
pub struct PluginRegistrar<'a> {
    event_handlers: &'a EventHandlerRegistry,
    dependencies: &'a DependStore,
}

impl<'a> PluginRegistrar<'a> {
    /// 为应用宿主创建一个插件注册器。
    pub fn new(event_handlers: &'a EventHandlerRegistry, dependencies: &'a DependStore) -> Self {
        Self {
            event_handlers,
            dependencies,
        }
    }

    /// 注册一个事件处理器。
    pub fn register_event_handler<K, Args, Kind, H>(&self, kind: K, handler: H)
    where
        K: Into<EventKind>,
        H: EventHandler<Args, Kind>,
    {
        self.event_handlers.register(kind, handler);
    }

    /// 向应用依赖容器中插入一个值。
    pub fn insert_dependency<T>(
        &self,
        value: T,
    ) -> Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>
    where
        T: std::any::Any + Send + Sync,
    {
        self.dependencies.insert(value)
    }

    /// 获取应用依赖容器。
    pub fn dependencies(&self) -> &DependStore {
        self.dependencies
    }
}

/// 原生应用插件的注册契约。
///
/// 插件只接触运行时注册器，不依赖具体的应用宿主或 HTTP 框架。
pub trait Plugin: Send + Sync + 'static {
    /// 将插件的事件处理器和其他应用资源注册到应用中。
    fn register(&self, registrar: &PluginRegistrar<'_>);
}
