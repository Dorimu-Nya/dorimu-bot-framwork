//! QQ bot SDK (webhook-first).

extern crate self as qqbot_sdk_core;

extern crate self as qqbot_sdk;

pub mod dependency;
pub use dependency::{resolve_dependency, DependencyProvider};

#[cfg(feature = "events")]
pub mod events;
#[cfg(feature = "events")]
pub use crate::events::common::{CommonMessage, FromCommonMessage, MessageFrom};
#[cfg(feature = "events")]
pub use crate::events::event::EventKind;
#[cfg(feature = "events")]
pub use crate::events::payload::{DispatchPayload, FromDispatchPayload};

#[cfg(feature = "openapi")]
pub mod openapi;
#[cfg(feature = "openapi")]
pub use openapi::error::{Error, Result};
#[cfg(feature = "openapi")]
pub use openapi::http::{HttpClient, RetryPolicy};
#[cfg(feature = "openapi")]
pub use openapi::models;
#[cfg(feature = "openapi")]
pub use openapi::{
    AnnouncesApi, ApiPermissionsApi, AuthConfig, C2cMessagesApi, ChannelPermissionsApi,
    ChannelsApi, ForumsApi, GuildsApi, HttpTokenProvider, InteractionsApi, MembersApi,
    MessageSettingsApi, MuteApi, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths, PinsApi,
    ReactionsApi, RolesApi, SchedulesApi, TokenManager, TokenProvider, UsersApi,
};

#[cfg(feature = "signature")]
pub mod signature;
#[cfg(feature = "signature")]
pub use signature::sign_webhook_validation;

#[cfg(feature = "events")]
pub mod event_handler;
#[cfg(feature = "events")]
pub use event_handler::{
    AsyncEventHandlerKind, BorrowedEventSyncHandlerKind, DynEventHandler, EventHandler,
    EventHandlerFuture, FromEventArg, PayloadEventArg, SyncEventHandlerKind,
};
