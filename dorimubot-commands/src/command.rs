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

impl CommandArgs for () {
    fn from_message(_message: &dyn CommonMessage) -> Self {}
}

macro_rules! impl_command_args {
    ($( $ty:ident ),+ $(,)?) => {
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

impl_command_args!(A1);
impl_command_args!(A1, A2);
impl_command_args!(A1, A2, A3);
impl_command_args!(A1, A2, A3, A4);
impl_command_args!(A1, A2, A3, A4, A5);
impl_command_args!(A1, A2, A3, A4, A5, A6);
impl_command_args!(A1, A2, A3, A4, A5, A6, A7);
impl_command_args!(A1, A2, A3, A4, A5, A6, A7, A8);

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
    type Args: CommandArgs;
    type Output: CommandOutput + Send + 'static;

    fn handle(&mut self, args: Self::Args) -> impl Future<Output = Self::Output> + Send + '_;
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

impl<C> CommandHandler<<C as AsyncCommand>::Args, AsyncStructCommandHandlerKind> for C
where
    C: AsyncCommand,
{
    fn into_dyn(self) -> DynCommandHandleFn {
        let handler = Arc::new(AsyncMutex::new(self));
        Arc::new(move |message| {
            let args = C::Args::from_message(message);
            let handler = Arc::clone(&handler);
            Box::pin(async move {
                let mut handler = handler.lock().await;
                let result = AsyncCommand::handle(&mut *handler, args).await;
                CommandOutput::into_output(result)
            })
        })
    }
}
