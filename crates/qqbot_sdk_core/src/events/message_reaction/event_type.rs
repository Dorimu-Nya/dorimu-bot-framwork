use super::reaction::MessageReaction;
use crate::events::event::Event;
use crate::events::payload::{DispatchPayload, FromDispatchPayload};
use serde::{Deserialize, Serialize};

#[qqbot_sdk_event_macros::event_kind]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum MessageReactionEventType {
    /// 为消息添加表情表态
    #[serde(rename = "MESSAGE_REACTION_ADD")]
    MessageReactionAdd(MessageReaction),
    /// 为消息删除表情表态
    #[serde(rename = "MESSAGE_REACTION_REMOVE")]
    MessageReactionRemove(MessageReaction),
}

impl FromDispatchPayload for MessageReaction {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::MessageReactionEventType(MessageReactionEventType::MessageReactionAdd(
                value,
            ))
            | Event::MessageReactionEventType(MessageReactionEventType::MessageReactionRemove(
                value,
            )) => Some(value.clone()),
            _ => None,
        }
    }
}
