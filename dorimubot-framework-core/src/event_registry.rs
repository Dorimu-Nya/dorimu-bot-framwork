use crate::event_handler::{into_dyn_event_handler, DynEventHandler, EventHandler};
use crate::events::EventSpec;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 按事件标记类型保存处理器的线程安全注册表。
#[derive(Clone, Default)]
pub(crate) struct EventHandlerRegistry {
    handlers: Arc<RwLock<HashMap<TypeId, Vec<DynEventHandler>>>>,
}

impl EventHandlerRegistry {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn register<E, H, Args, Mode>(&self, _: E, handler: H)
    where
        E: EventSpec,
        H: EventHandler<E, Args, Mode>,
    {
        let handler = into_dyn_event_handler::<E, H, Args, Mode>(handler);
        self.handlers
            .write()
            .expect("事件处理器注册表的写锁已中毒")
            .entry(TypeId::of::<E>())
            .or_default()
            .push(handler);
    }

    pub(crate) fn handlers_for<E>(&self) -> Vec<DynEventHandler>
    where
        E: EventSpec,
    {
        self.handlers
            .read()
            .expect("事件处理器注册表的读锁已中毒")
            .get(&TypeId::of::<E>())
            .cloned()
            .unwrap_or_default()
    }
}
