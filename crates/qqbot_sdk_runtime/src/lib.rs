//! Event handling, dependency injection, and native plugin APIs for qqbot_sdk.

mod depend;
mod dependency;
mod event_handler;
mod event_registry;
mod plugin;

pub use depend::{Depend, DependArg, DependStore};
pub use dependency::{resolve_dependency, DependencyProvider};
pub use event_handler::{
    AsyncEventHandlerKind, BorrowedEventSyncHandlerKind, DynEventHandler, EventHandler,
    EventHandlerFuture, FromEventArg, PayloadEventArg, SyncEventHandlerKind,
};
pub use event_registry::EventHandlerRegistry;
pub use plugin::{Plugin, PluginRegistrar};
