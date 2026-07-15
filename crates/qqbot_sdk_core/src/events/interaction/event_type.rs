use super::models::Interaction;
use crate::events::event::Event;
use crate::events::payload::{DispatchPayload, FromDispatchPayload};
use serde::{Deserialize, Serialize};

/// 互动事件
#[qqbot_sdk_event_macros::event_kind]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum InteractionEventType {
    /// 创建互动事件, 用户点击了消息体的回调按钮触发
    #[serde(rename = "INTERACTION_CREATE")]
    InteractionCreate(Interaction),
}

impl FromDispatchPayload for Interaction {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::InteractionEventType(InteractionEventType::InteractionCreate(value)) => {
                Some(value.clone())
            }
            _ => None,
        }
    }
}
