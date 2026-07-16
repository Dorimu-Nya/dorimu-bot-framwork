use super::c2c::event_type::{C2cEventType, C2cEventTypeKind};
use super::group::event_type::{GroupEventType, GroupEventTypeKind};
use super::guild::event_type::{
    ForumEventType, ForumEventTypeKind, GuildEventType, GuildEventTypeKind,
};
use super::interaction::event_type::{InteractionEventType, InteractionEventTypeKind};
use super::message_reaction::event_type::{MessageReactionEventType, MessageReactionEventTypeKind};
use serde::{Deserialize, Serialize};

/// 全部事件类型汇总分类
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    C2cEventType(C2cEventType),
    GroupEventType(GroupEventType),
    GuildEventType(GuildEventType),
    ForumEventType(ForumEventType),
    InteractionEventType(InteractionEventType),
    MessageReactionEventType(MessageReactionEventType),
}

/// 全部事件注册键的统一表示。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventKind {
    C2c(C2cEventTypeKind),
    Group(GroupEventTypeKind),
    Guild(GuildEventTypeKind),
    Forum(ForumEventTypeKind),
    Interaction(InteractionEventTypeKind),
    MessageReaction(MessageReactionEventTypeKind),
}

macro_rules! impl_event_kind_from {
    ($kind:ty, $variant:ident) => {
        impl From<$kind> for EventKind {
            fn from(kind: $kind) -> Self {
                Self::$variant(kind)
            }
        }
    };
}

impl_event_kind_from!(C2cEventTypeKind, C2c);
impl_event_kind_from!(GroupEventTypeKind, Group);
impl_event_kind_from!(GuildEventTypeKind, Guild);
impl_event_kind_from!(ForumEventTypeKind, Forum);
impl_event_kind_from!(InteractionEventTypeKind, Interaction);
impl_event_kind_from!(MessageReactionEventTypeKind, MessageReaction);
