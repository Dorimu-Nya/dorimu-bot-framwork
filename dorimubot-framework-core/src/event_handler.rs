use crate::events::EventSpec;
use std::any::{type_name, Any};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// 注册表调用处理器后统一得到的返回类型。
///
/// 异步处理器返回的 Future 会装进这里；同步处理器执行完后则返回一个
/// 立即完成的空 Future。这样分发时不需要再区分同步和异步处理器。
pub(crate) type EventHandlerFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// 注册表最终保存的统一处理器类型。
///
/// 注册时，下面几种函数或闭包都会被转换成这个类型：
///
/// - 同步零参数：`Fn()`
/// - 异步零参数：`Fn() -> Future`
/// - 同步有参数：`Fn(A1, ..., An)`
/// - 异步有参数：`Fn(A1, ..., An) -> Future`
///
/// 具体的事件载荷和函数参数类型在这里会被擦除。调用时再根据事件标记
/// `E` 将载荷还原成 `E::Payload`，并转换出处理器需要的各个参数。
pub(crate) type DynEventHandler =
    Arc<dyn Fn(&(dyn Any + Send + Sync)) -> EventHandlerFuture + Send + Sync>;

/// 告诉编译器当前适配的是同步处理器，不代表任何事件类型。
#[doc(hidden)]
pub struct SyncHandlerMode;

/// 告诉编译器当前适配的是异步处理器，不代表任何事件类型。
#[doc(hidden)]
pub struct AsyncHandlerMode;

/// 将一种具体签名的函数或闭包转换成 [`DynEventHandler`]。
///
/// `Args` 记录处理器的参数列表，`Mode` 用于区分同步和异步实现。
trait EventHandlerAdapter<E, Args, Mode>: Send + Sync + 'static
where
    E: EventSpec,
{
    fn into_dyn(self) -> DynEventHandler;
}

/// 可以处理事件 `E` 的函数或闭包。
///
/// 处理器可以没有参数，也可以接收最多八个按值参数。对于每个参数
/// `A`，事件载荷必须满足 `E::Payload: Into<A>`。同一个处理器的多个
/// 参数分别由同一份事件载荷克隆并转换得到。
#[allow(private_bounds)]
pub trait EventHandler<E, Args, Mode>: EventHandlerAdapter<E, Args, Mode>
where
    E: EventSpec,
{
}

impl<T, E, Args, Mode> EventHandler<E, Args, Mode> for T
where
    E: EventSpec,
    T: EventHandlerAdapter<E, Args, Mode>,
{
}

/// 注册处理器时的最后一步：将已经通过类型检查的函数或闭包转换成
/// 注册表统一保存的 [`DynEventHandler`]。
pub(crate) fn into_dyn_event_handler<E, H, Args, Mode>(handler: H) -> DynEventHandler
where
    E: EventSpec,
    H: EventHandler<E, Args, Mode>,
{
    <H as EventHandlerAdapter<E, Args, Mode>>::into_dyn(handler)
}

/// 将分发器传来的已擦除载荷还原成当前事件声明的载荷类型。
fn expect_payload<E>(payload: &(dyn Any + Send + Sync)) -> &E::Payload
where
    E: EventSpec,
{
    payload.downcast_ref::<E::Payload>().unwrap_or_else(|| {
        panic!(
            "事件分发器传入了错误的载荷类型，期望 {}",
            type_name::<E::Payload>()
        )
    })
}

/// 为不同参数数量生成处理器适配实现。
///
/// Rust 没有可变数量泛型，因此零到八个参数需要分别实现。这个宏只负责
/// 展开这些重复实现，不参与事件分发：
///
/// - 空调用 `impl_event_handler_adapter!()` 生成同步、异步零参数实现；
/// - 有参调用生成对应参数数量的同步、异步实现；
/// - 每个参数都要求 `E::Payload: Into<参数类型>`；
/// - 调用处理器前，为每个参数克隆一次载荷并执行 `Into` 转换；
/// - 最后把处理器统一包装成 [`DynEventHandler`]。
macro_rules! impl_event_handler_adapter {
    () => {
        // `Fn()` 被包装后忽略事件载荷，执行完成后返回空 Future。
        impl<F, E> EventHandlerAdapter<E, (), SyncHandlerMode> for F
        where
            E: EventSpec,
            F: Fn() + Send + Sync + 'static,
        {
            fn into_dyn(self) -> DynEventHandler {
                Arc::new(move |_| {
                    self();
                    Box::pin(async {})
                })
            }
        }

        // `Fn() -> Future` 被包装后忽略事件载荷，并返回原 Future。
        impl<F, Fut, E> EventHandlerAdapter<E, (), AsyncHandlerMode> for F
        where
            E: EventSpec,
            F: Fn() -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static,
        {
            fn into_dyn(self) -> DynEventHandler {
                Arc::new(move |_| Box::pin(self()))
            }
        }
    };
    ($( $arg_type:ident => $argument:ident ),+ $(,)?) => {
        // 同步有参处理器：还原载荷，转换参数，调用后返回空 Future。
        impl<F, E, $($arg_type),+>
            EventHandlerAdapter<E, ($($arg_type,)+), SyncHandlerMode> for F
        where
            E: EventSpec,
            F: Fn($($arg_type),+) + Send + Sync + 'static,
            $(E::Payload: Into<$arg_type>,)+
            $($arg_type: Send + 'static,)+
        {
            fn into_dyn(self) -> DynEventHandler {
                Arc::new(move |erased_payload| {
                    let payload = expect_payload::<E>(erased_payload);
                    $(let $argument: $arg_type = E::Payload::clone(payload).into();)+
                    self($($argument),+);
                    Box::pin(async {})
                })
            }
        }

        // 异步有参处理器：还原载荷，转换参数，并返回处理器产生的 Future。
        impl<F, Fut, E, $($arg_type),+>
            EventHandlerAdapter<E, ($($arg_type,)+), AsyncHandlerMode> for F
        where
            E: EventSpec,
            F: Fn($($arg_type),+) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static,
            $(E::Payload: Into<$arg_type>,)+
            $($arg_type: Send + 'static,)+
        {
            fn into_dyn(self) -> DynEventHandler {
                Arc::new(move |erased_payload| {
                    let payload = expect_payload::<E>(erased_payload);
                    $(let $argument: $arg_type = E::Payload::clone(payload).into();)+
                    Box::pin(self($($argument),+))
                })
            }
        }
    };
}

// 生成零参数处理器的两种实现。
impl_event_handler_adapter!();

// 生成一到八个参数处理器的同步、异步实现。
impl_event_handler_adapter!(A1 => a1);
impl_event_handler_adapter!(A1 => a1, A2 => a2);
impl_event_handler_adapter!(A1 => a1, A2 => a2, A3 => a3);
impl_event_handler_adapter!(A1 => a1, A2 => a2, A3 => a3, A4 => a4);
impl_event_handler_adapter!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5);
impl_event_handler_adapter!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6);
impl_event_handler_adapter!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6, A7 => a7);
impl_event_handler_adapter!(A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5, A6 => a6, A7 => a7, A8 => a8);
