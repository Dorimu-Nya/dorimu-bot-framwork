use super::models::{
    C2cMessage, C2cMsgReceiveEvent, C2cMsgRejectEvent, FriendAddEvent, FriendDelEvent,
};
use crate::events::event::Event;
use crate::events::payload::{DispatchPayload, FromDispatchPayload};
use serde::{Deserialize, Serialize};

/// 单聊事件
#[qqbot_sdk_event_macros::event_kind]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum C2cEventType {
    /// C2C 消息事件
    #[serde(rename = "C2C_MESSAGE_CREATE")]
    C2cMessageCreate(C2cMessage),
    /// C2C 添加好友
    #[serde(rename = "FRIEND_ADD")]
    FriendAdd(FriendAddEvent),
    /// C2C 删除好友
    #[serde(rename = "FRIEND_DEL")]
    FriendDel(FriendDelEvent),
    /// C2C 关闭消息推送
    #[serde(rename = "C2C_MSG_REJECT")]
    C2cMsgReject(C2cMsgRejectEvent),
    /// C2C 打开消息推送
    #[serde(rename = "C2C_MSG_RECEIVE")]
    C2cMsgReceive(C2cMsgReceiveEvent),
}

impl FromDispatchPayload for C2cMessage {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::C2cEventType(C2cEventType::C2cMessageCreate(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for FriendAddEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::C2cEventType(C2cEventType::FriendAdd(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for FriendDelEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::C2cEventType(C2cEventType::FriendDel(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for C2cMsgRejectEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::C2cEventType(C2cEventType::C2cMsgReject(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for C2cMsgReceiveEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::C2cEventType(C2cEventType::C2cMsgReceive(value)) => Some(value.clone()),
            _ => None,
        }
    }
}
