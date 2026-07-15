//! QQ bot SDK (webhook-first).

extern crate self as qqbot_sdk_core;

extern crate self as qqbot_sdk;

#[cfg(feature = "events")]
pub mod events;
pub use crate::events::common::{CommonMessage, FromCommonMessage, MessageFrom};
pub use crate::events::payload::{DispatchPayload, FromDispatchPayload};

#[cfg(feature = "openapi")]
pub mod openapi;
pub use openapi::error::{Error, Result};
pub use openapi::http::{HttpClient, RetryPolicy};
pub use openapi::models;
pub use openapi::{
    AnnouncesApi, ApiPermissionsApi, AuthConfig, C2cMessagesApi, ChannelPermissionsApi,
    ChannelsApi, ForumsApi, GuildsApi, HttpTokenProvider, InteractionsApi, MembersApi,
    MessageSettingsApi, MuteApi, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths, PinsApi,
    ReactionsApi, RolesApi, SchedulesApi, TokenManager, TokenProvider, UsersApi,
};

#[cfg(feature = "signature")]
pub mod signature;
pub use signature::sign_webhook_validation;

pub mod event_handler;
pub mod event_registry_key;
pub use event_handler::{
    AsyncEventHandlerKind, BorrowedEventSyncHandlerKind, DynEventHandler, EventHandler,
    EventHandlerFuture, SyncEventHandlerKind,
};
pub use event_registry_key::KindRegistryKey;
