use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// 事件处理器统一使用的异步返回类型。
pub type EventHandlerFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// 擦除具体参数后的事件处理器，用于保存在事件注册表中。
pub type DynEventHandler =
    Arc<dyn Fn(&(dyn Any + Send + Sync)) -> EventHandlerFuture + Send + Sync>;

/// 同步事件函数的适配标记。
pub struct SyncEventHandlerKind;
/// 异步事件函数的适配标记。
pub struct AsyncEventHandlerKind;
/// 借用事件参数的同步函数适配标记。
pub struct BorrowedEventSyncHandlerKind;

/// 将普通函数适配为统一事件处理函数的 trait。
///
/// `Data` 是事件的原始 data 类型，`Args` 由函数参数推导，`Kind`
/// 将同步函数与异步函数分开。owned 参数可以是任意满足
/// `Data: Into<Arg>` 的类型。
pub trait EventHandler<Data, Args, Kind>: Send + Sync + 'static {
    fn into_dyn(self) -> DynEventHandler;
}

impl<F, Data> EventHandler<Data, (), SyncEventHandlerKind> for F
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

impl<F, Fut, Data> EventHandler<Data, (), AsyncEventHandlerKind> for F
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    fn into_dyn(self) -> DynEventHandler {
        Arc::new(move |_| Box::pin(self()))
    }
}

impl<F, Data> EventHandler<Data, (Data,), BorrowedEventSyncHandlerKind> for F
where
    F: Fn(&Data) + Send + Sync + 'static,
    Data: Any + Send + Sync,
{
    fn into_dyn(self) -> DynEventHandler {
        Arc::new(move |event_data| {
            if let Some(value) = event_data.downcast_ref::<Data>() {
                self(value);
            }
            Box::pin(async {})
        })
    }
}

impl<F, Data, Arg> EventHandler<Data, (Arg,), SyncEventHandlerKind> for F
where
    F: Fn(Arg) + Send + Sync + 'static,
    Data: Any + Clone + Into<Arg> + Send + Sync,
    Arg: Send + 'static,
{
    fn into_dyn(self) -> DynEventHandler {
        Arc::new(move |event_data| {
            if let Some(value) = event_data.downcast_ref::<Data>().cloned() {
                self(value.into());
            }
            Box::pin(async {})
        })
    }
}

impl<F, Fut, Data, Arg> EventHandler<Data, (Arg,), AsyncEventHandlerKind> for F
where
    F: Fn(Arg) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
    Data: Any + Clone + Into<Arg> + Send + Sync,
    Arg: Send + 'static,
{
    fn into_dyn(self) -> DynEventHandler {
        Arc::new(move |event_data| {
            let Some(value) = event_data.downcast_ref::<Data>().cloned() else {
                return Box::pin(async {});
            };
            Box::pin(self(value.into()))
        })
    }
}
