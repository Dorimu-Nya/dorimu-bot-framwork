use super::super::common::Attachment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 群消息作者
pub struct GroupAuthor {
    /// 成员 openid
    pub member_openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 群 @ 消息
///
/// 触发场景：群内用户 @ 机器人发送消息
pub struct GroupAtMessage {
    /// 平台方消息ID，可以用于被动消息发送
    pub id: String,
    /// 发送者
    pub author: GroupAuthor,
    /// 文本消息内容
    pub content: Option<String>,
    /// 消息生产时间（RFC3339）
    pub timestamp: Option<String>,
    /// 群 openid
    pub group_openid: String,
    /// 富媒体文件附件，文件类型："图片，语音，视频，文件"
    pub attachments: Option<Vec<Attachment>>,
    #[serde(default)]
    /// 消息序列
    pub msg_seq: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 机器人被添加到群
pub struct GroupAddRobotEvent {
    /// 操作时间戳
    pub timestamp: i64,
    /// 群 openid
    pub group_openid: String,
    /// 操作成员 openid
    pub op_member_openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 机器人被移出群
pub struct GroupDelRobotEvent {
    /// 操作时间戳
    pub timestamp: i64,
    /// 群 openid
    pub group_openid: String,
    /// 操作成员 openid
    pub op_member_openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 拒绝机器人主动消息
pub struct GroupMsgRejectEvent {
    /// 操作时间戳
    pub timestamp: i64,
    /// 群 openid
    pub group_openid: String,
    /// 操作成员 openid
    pub op_member_openid: String,
}
