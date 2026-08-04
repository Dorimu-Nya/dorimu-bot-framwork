use crate::{DynEventHandler, EventHandler};
use qqbot_rust_sdk::events::payload::event::EventKind;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 按事件类型保存处理器的线程安全注册表。
#[derive(Clone, Default)]
pub(crate) struct EventHandlerRegistry {
    handlers: Arc<RwLock<HashMap<EventKind, Vec<DynEventHandler>>>>,
}

impl EventHandlerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn register<K, Args, Kind, H>(&self, kind: K, handler: H)
    where
        K: Into<EventKind>,
        H: EventHandler<Args, Kind>,
    {
        self.register_dyn(kind, handler.into_dyn());
    }

    fn register_dyn<K>(&self, kind: K, handler: DynEventHandler)
    where
        K: Into<EventKind>,
    {
        self.handlers
            .write()
            .unwrap()
            .entry(kind.into())
            .or_default()
            .push(handler);
    }

    pub(crate) fn get_handlers<K>(&self, kind: K) -> Vec<DynEventHandler>
    where
        K: Into<EventKind>,
    {
        self.handlers
            .read()
            .unwrap()
            .get(&kind.into())
            .cloned()
            .unwrap_or_default()
    }
}
