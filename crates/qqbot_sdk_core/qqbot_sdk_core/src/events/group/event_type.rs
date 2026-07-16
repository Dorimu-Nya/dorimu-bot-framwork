use super::models::{GroupAddRobotEvent, GroupAtMessage, GroupDelRobotEvent, GroupMsgRejectEvent};
use crate::events::event::Event;
use crate::events::payload::{DispatchPayload, FromDispatchPayload};
use serde::{Deserialize, Serialize};

/// 群事件
#[qqbot_sdk_core_macros::event_kind]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum GroupEventType {
    /// 群消息事件 AT 事件
    #[serde(rename = "GROUP_AT_MESSAGE_CREATE")]
    GroupAtMessageCreate(GroupAtMessage),
    /// 群添加机器人
    #[serde(rename = "GROUP_AT_ROBOT")]
    GroupAddRobot(GroupAddRobotEvent),
    /// 群移除机器人
    #[serde(rename = "GROUP_DEL_ROBOT")]
    GroupDelRobot(GroupDelRobotEvent),
    /// 群开启消息推送
    #[serde(rename = "GROUP_MSG_RECEIVE")]
    GroupMsgReceive(GroupAddRobotEvent),
    /// 群关闭消息推送
    #[serde(rename = "GROUP_MSG_REJECT")]
    GroupMsgReject(GroupMsgRejectEvent),
    /// 消息授权状态变更
    #[serde(rename = "SUBSCRIBE_MESSAGE_STATUS")]
    SubscribeMessageStatus,
}

impl FromDispatchPayload for GroupAtMessage {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GroupEventType(GroupEventType::GroupAtMessageCreate(value)) => {
                Some(value.clone())
            }
            _ => None,
        }
    }
}

impl FromDispatchPayload for GroupAddRobotEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GroupEventType(GroupEventType::GroupAddRobot(value))
            | Event::GroupEventType(GroupEventType::GroupMsgReceive(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for GroupDelRobotEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GroupEventType(GroupEventType::GroupDelRobot(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for GroupMsgRejectEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GroupEventType(GroupEventType::GroupMsgReject(value)) => Some(value.clone()),
            _ => None,
        }
    }
}
