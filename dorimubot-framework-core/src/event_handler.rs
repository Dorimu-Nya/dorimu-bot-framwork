use qqbot_rust_sdk::events::payload::payload::DispatchPayload;
use serde::de::DeserializeOwned;
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// 事件处理器统一使用的异步返回类型。
pub type EventHandlerFuture<'a> = Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

/// 擦除具体参数后的事件处理器，用于保存在事件注册表中。
pub type DynEventHandler =
    Arc<dyn for<'a> Fn(EventHandlerInput<'a>) -> EventHandlerFuture<'a> + Send + Sync>;

/// 一次事件处理调用可使用的参数来源。
///
/// `payload` 是完整的下行载荷，`event_data` 是当前事件枚举变体携带的数据。
#[derive(Clone, Copy)]
pub struct EventHandlerInput<'a> {
    payload: &'a DispatchPayload,
    event_data: &'a (dyn Any + Send + Sync),
}

impl<'a> EventHandlerInput<'a> {
    pub(crate) fn new(
        payload: &'a DispatchPayload,
        event_data: &'a (dyn Any + Send + Sync),
    ) -> Self {
        Self {
            payload,
            event_data,
        }
    }

    fn get<T: Any>(&self) -> Option<&'a T> {
        (self.payload as &dyn Any)
            .downcast_ref::<T>()
            .or_else(|| self.event_data.downcast_ref::<T>())
    }
}

/// 同步事件函数的适配标记。
pub struct SyncEventHandlerKind;
/// 异步事件函数的适配标记。
pub struct AsyncEventHandlerKind;
/// 借用事件参数的同步函数适配标记。
pub struct BorrowedEventSyncHandlerKind;

/// 将普通函数适配为统一事件处理函数的 trait。
///
/// `Args` 由函数参数推导，`Kind` 将同步函数与异步函数分开，避免 trait
/// coherence 冲突。每个参数按实际类型从完整 [`DispatchPayload`] 或当前
/// 事件枚举变体携带的数据中提取。
pub trait EventHandler<Args, Kind>: Send + Sync + 'static {
    fn into_dyn(self) -> DynEventHandler;
}

macro_rules! impl_event_handler {
    () => {
        impl<F> EventHandler<(), SyncEventHandlerKind> for F
        where
            F: Fn() + Send + Sync + 'static,
        {
            fn into_dyn(self) -> DynEventHandler {
                Arc::new(move |_| {
                    self();
                    Box::pin(async {})
                })
            }
        }

        impl<F, Fut> EventHandler<(), AsyncEventHandlerKind> for F
        where
            F: Fn() -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static,
        {
            fn into_dyn(self) -> DynEventHandler {
                Arc::new(move |_| Box::pin(self()))
            }
        }
    };
    ($( $ty:ident => $var:ident ),+ $(,)?) => {
        // `DeserializeOwned` is only used to exclude reference types from these
        // generic parameters and keep borrowed/owned handler impls disjoint.
        // Parameter extraction itself is a direct `Any` downcast and never
        // serializes or deserializes the event data.
        impl<F, $($ty),+> EventHandler<($($ty,)+), BorrowedEventSyncHandlerKind> for F
        where
            F: Fn($(& $ty),+) + Send + Sync + 'static,
            $($ty: Any + DeserializeOwned + Send + Sync,)+
        {
            fn into_dyn(self) -> DynEventHandler {
                Arc::new(move |input| {
                    $(
                        let Some($var) = input.get::<$ty>() else {
                            return Box::pin(async {});
                        };
                    )+
                    self($($var),+);
                    Box::pin(async {})
                })
            }
        }

        impl<F, $($ty),+> EventHandler<($($ty,)+), SyncEventHandlerKind> for F
        where
            F: Fn($($ty),+) + Send + Sync + 'static,
            $($ty: Any + Clone + DeserializeOwned + Send,)+
        {
            fn into_dyn(self) -> DynEventHandler {
                Arc::new(move |input| {
                    $(
                        let Some($var) = input.get::<$ty>().cloned() else {
                            return Box::pin(async {});
                        };
                    )+
                    self($($var),+);
                    Box::pin(async {})
                })
            }
        }

        impl<F, Fut, $($ty),+> EventHandler<($($ty,)+), AsyncEventHandlerKind> for F
        where
            F: Fn($($ty),+) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static,
            $($ty: Any + Clone + DeserializeOwned + Send,)+
        {
            fn into_dyn(self) -> DynEventHandler {
                Arc::new(move |input| {
                    $(
                        let Some($var) = input.get::<$ty>().cloned() else {
                            return Box::pin(async {});
                        };
                    )+
                    let future = self($($var),+);
                    Box::pin(future)
                })
            }
        }
    };
}

impl_event_handler!();
impl_event_handler!(A1 => a1);
impl_event_handler!(A1 => a1, A2 => a2);
impl_event_handler!(A1 => a1, A2 => a2, A3 => a3);
impl_event_handler!(A1 => a1, A2 => a2, A3 => a3, A4 => a4);
impl_event_handler!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5);
impl_event_handler!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6);
impl_event_handler!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6, A7 => a7);
impl_event_handler!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6, A7 => a7, A8 => a8);
