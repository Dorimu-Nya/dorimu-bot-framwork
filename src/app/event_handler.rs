use crate::events::payload::{DispatchPayload, FromDispatchPayload};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// 事件处理器统一使用的异步返回类型。
pub type EventHandlerFuture<'a> = Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

/// 擦除具体参数后的事件处理器，用于保存在宏生成的静态 Kind 容器中。
pub type DynEventHandler =
    Arc<dyn for<'a> Fn(&'a DispatchPayload) -> EventHandlerFuture<'a> + Send + Sync>;

/// 同步事件函数的适配标记。
pub struct SyncEventHandlerKind;
/// 异步事件函数的适配标记。
pub struct AsyncEventHandlerKind;
/// 借用事件参数的同步函数适配标记。
pub struct BorrowedEventSyncHandlerKind;

/// 将普通函数适配为统一事件处理函数的 trait。
///
/// `Args` 由函数参数推导，`Kind` 将同步函数与异步函数分开，避免 trait
/// coherence 冲突。具体事件参数通过 [`FromDispatchPayload`] 从 webhook 载荷提取。
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
        impl<F, $($ty),+> EventHandler<($($ty,)+), BorrowedEventSyncHandlerKind> for F
        where
            F: Fn($(& $ty),+) + Send + Sync + 'static,
            $($ty: FromDispatchPayload + Send + Sync + 'static,)+
        {
            fn into_dyn(self) -> DynEventHandler {
                Arc::new(move |payload| {
                    $(
                        let Some($var) = <$ty as FromDispatchPayload>::from(payload) else {
                            return Box::pin(async {});
                        };
                    )+
                    self($(& $var),+);
                    Box::pin(async {})
                })
            }
        }

        impl<F, Fut, $($ty),+> EventHandler<($($ty,)+), AsyncEventHandlerKind> for F
        where
            F: Fn($($ty),+) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static,
            $($ty: FromDispatchPayload + Send + Sync + 'static,)+
        {
            fn into_dyn(self) -> DynEventHandler {
                Arc::new(move |payload| {
                    $(
                        let Some($var) = <$ty as FromDispatchPayload>::from(payload) else {
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
