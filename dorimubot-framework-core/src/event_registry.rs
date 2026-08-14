use super::event_handler::{DynEventHandler, EventHandler};
use crate::TypedEventKind;
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

    pub(crate) fn register<K, Data, Args, Kind, H>(
        &self,
        event: TypedEventKind<K, Data>,
        handler: H,
    ) where
        K: Into<EventKind>,
        H: EventHandler<Data, Args, Kind>,
    {
        self.register_dyn(event.into_kind(), handler.into_dyn());
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
