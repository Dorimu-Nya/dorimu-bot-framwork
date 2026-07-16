pub use qqbot_sdk_commands_macros::command;

pub mod common;
pub mod defining;
pub mod replying;
pub mod store;

pub use common::{CommonMessage, FromCommonMessage, MessageFrom};
pub use defining::{
    wrap_command_handle_fn, CommandDef, CommandHandleFn, CommandHandleFuture, CommandHandler,
    CommandOutput, DynCommandHandleFn, FromCommandArg, MessageCommandArg,
};
pub use replying::{ReplyingMessage, ReplyingType};
pub use store::CommandsStore;
