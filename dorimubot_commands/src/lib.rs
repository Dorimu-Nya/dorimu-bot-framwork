pub use dorimubot_commands_macros::command;
#[doc(hidden)]
pub use inventory;

mod command_plugin;
pub mod common;
pub mod defining;
pub mod replying;
pub mod store;

pub use command_plugin::CommandPlugin;
pub use common::{CommonMessage, FromCommonMessage, MessageFrom};
pub use defining::{
    wrap_command_handle_fn, CommandDef, CommandHandleFn, CommandHandleFuture, CommandHandler,
    CommandOutput, DynCommandHandleFn,
};
pub use replying::{ReplyingMessage, ReplyingType};
pub use store::CommandsStore;
