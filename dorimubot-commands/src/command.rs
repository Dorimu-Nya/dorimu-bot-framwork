use crate::arity::for_each_command_arity;
use crate::common::{CommonMessage, FromCommonMessage};
use crate::defining::{CommandHandler, CommandOutput, DynCommandHandleFn};
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

/// 结构体同步命令的适配标记。
#[doc(hidden)]
pub struct StructCommandHandlerKind;

/// 结构体异步命令的适配标记。
#[doc(hidden)]
pub struct AsyncStructCommandHandlerKind;

/// 可从一条消息中提取的命令参数列表。
///
/// 该 trait 已为 `()` 以及包含 1～8 个 [`FromCommonMessage`] 参数的元组实现。
#[doc(hidden)]
pub trait CommandArgs: Send + 'static {
    fn from_message(message: &dyn CommonMessage) -> Self;
}

/// 可从一条消息中提取的异步命令参数列表。
///
/// 与 [`CommandArgs`] 不同，该 trait 允许参数借用消息本身。
#[doc(hidden)]
pub trait AsyncCommandArgs<'a>: Send + 'a {
    fn from_message(message: &'a dyn CommonMessage) -> Self;
}

macro_rules! impl_command_args {
    () => {
        impl CommandArgs for () {
            fn from_message(_message: &dyn CommonMessage) -> Self {}
        }
    };
    ($( $ty:ident => $var:ident ),+ $(,)?) => {
        impl<$($ty),+> CommandArgs for ($($ty,)+)
        where
            $(
                $ty: for<'a> FromCommonMessage<'a> + Send + 'static,
            )+
        {
            fn from_message(message: &dyn CommonMessage) -> Self {
                (
                    $(
                        <$ty as FromCommonMessage<'_>>::from(message),
                    )+
                )
            }
        }
    };
}

for_each_command_arity!(impl_command_args);

macro_rules! impl_async_command_args {
    () => {
        impl AsyncCommandArgs<'_> for () {
            fn from_message(_message: &dyn CommonMessage) -> Self {}
        }
    };
    ($( $ty:ident => $var:ident ),+ $(,)?) => {
        impl<'a, $($ty),+> AsyncCommandArgs<'a> for ($($ty,)+)
        where
            $(
                $ty: FromCommonMessage<'a> + Send + 'a,
            )+
        {
            fn from_message(message: &'a dyn CommonMessage) -> Self {
                (
                    $(
                        <$ty as FromCommonMessage<'a>>::from(message),
                    )+
                )
            }
        }
    };
}

for_each_command_arity!(impl_async_command_args);

/// 可以直接注册到 [`crate::CommandPlugin`] 的同步命令结构体。
///
/// 参数列表通过 [`Command::Args`] 指定：零参数使用 `()`，一个参数使用 `(A1,)`，
/// 多个参数使用 `(A1, A2, ...)`。每个参数都会通过 [`FromCommonMessage`] 自动提取。
pub trait Command: Send + 'static {
    type Args: CommandArgs;
    type Output: CommandOutput + Send + 'static;

    fn handle(&mut self, args: Self::Args) -> Self::Output;
}

/// 可以直接注册到 [`crate::CommandPlugin`] 的异步命令结构体。
///
/// 参数规则与 [`Command`] 相同。处理同一命令实例时会持有异步互斥锁，因此实现可以在
/// `.await` 前后安全地访问 `&mut self`。
pub trait AsyncCommand: Send + 'static {
    type Args<'a>: AsyncCommandArgs<'a>;
    type Output: CommandOutput + Send + 'static;

    fn handle<'a>(
        &'a mut self,
        args: Self::Args<'a>,
    ) -> impl Future<Output = Self::Output> + Send + 'a;
}

impl<C> CommandHandler<<C as Command>::Args, StructCommandHandlerKind> for C
where
    C: Command,
{
    fn into_dyn(self) -> DynCommandHandleFn {
        let handler = Mutex::new(self);
        Arc::new(move |message| {
            let args = C::Args::from_message(message);
            let mut handler = handler
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let result = Command::handle(&mut *handler, args);
            Box::pin(async move { CommandOutput::into_output(result) })
        })
    }
}

impl<C> CommandHandler<(), AsyncStructCommandHandlerKind> for C
where
    C: AsyncCommand,
{
    fn into_dyn(self) -> DynCommandHandleFn {
        let handler = Arc::new(AsyncMutex::new(self));
        Arc::new(move |message| {
            let handler = Arc::clone(&handler);
            Box::pin(async move {
                let mut handler = handler.lock().await;
                let args = C::Args::from_message(message);
                let result = AsyncCommand::handle(&mut *handler, args).await;
                CommandOutput::into_output(result)
            })
        })
    }
}
