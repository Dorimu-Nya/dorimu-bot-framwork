use qqbot_sdk_core::events::c2c::models::C2cMessage;
use qqbot_sdk_core::events::common::Attachment;
use qqbot_sdk_core::events::group::models::GroupAtMessage;

/// 消息来源，用于统一命令处理的消息抽象。
pub enum MessageFrom {
    C2c,
    Group,
}

/// 命令处理器使用的私聊和群聊消息抽象。
pub trait CommonMessage: Sync {
    fn get_id(&self) -> &String;
    fn get_content(&self) -> &Option<String>;
    fn get_author_openid(&self) -> &String;
    fn get_timestamp(&self) -> &Option<String>;
    fn get_attachments(&self) -> &Option<Vec<Attachment>>;
    fn get_msg_seq(&self) -> &Option<u64>;
    fn get_message_from_type(&self) -> MessageFrom;
    /// 返回当前场景的 openid：私聊为用户 id，群聊为群 id。
    fn get_scene_openid(&self) -> &String;
}

impl CommonMessage for C2cMessage {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_content(&self) -> &Option<String> {
        &self.content
    }

    fn get_author_openid(&self) -> &String {
        &self.author.user_openid
    }

    fn get_timestamp(&self) -> &Option<String> {
        &self.timestamp
    }

    fn get_attachments(&self) -> &Option<Vec<Attachment>> {
        &self.attachments
    }

    fn get_msg_seq(&self) -> &Option<u64> {
        &self.msg_seq
    }

    fn get_message_from_type(&self) -> MessageFrom {
        MessageFrom::C2c
    }

    fn get_scene_openid(&self) -> &String {
        &self.author.user_openid
    }
}

impl CommonMessage for GroupAtMessage {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_content(&self) -> &Option<String> {
        &self.content
    }

    fn get_author_openid(&self) -> &String {
        &self.author.member_openid
    }

    fn get_timestamp(&self) -> &Option<String> {
        &self.timestamp
    }

    fn get_attachments(&self) -> &Option<Vec<Attachment>> {
        &self.attachments
    }

    fn get_msg_seq(&self) -> &Option<u64> {
        &self.msg_seq
    }

    fn get_message_from_type(&self) -> MessageFrom {
        MessageFrom::Group
    }

    fn get_scene_openid(&self) -> &String {
        &self.group_openid
    }
}

/// 从统一命令消息中提取处理器参数。
pub trait FromCommonMessage<'a>: Sized {
    fn from(req: &'a dyn CommonMessage) -> Self;
}

impl<'a> FromCommonMessage<'a> for &'a dyn CommonMessage {
    fn from(req: &'a dyn CommonMessage) -> Self {
        req
    }
}

impl<'a> FromCommonMessage<'a> for &'a Option<Vec<Attachment>> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        req.get_attachments()
    }
}

impl FromCommonMessage<'_> for Option<Vec<Attachment>> {
    fn from(req: &dyn CommonMessage) -> Self {
        req.get_attachments().clone()
    }
}

impl<'a> FromCommonMessage<'a> for Option<Vec<&'a str>> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        req.get_content()
            .as_ref()
            .map(|message| message.split_whitespace().collect())
    }
}

impl FromCommonMessage<'_> for Option<Vec<String>> {
    fn from(req: &dyn CommonMessage) -> Self {
        req.get_content().as_ref().map(|message| {
            message
                .split_whitespace()
                .map(ToString::to_string)
                .collect()
        })
    }
}

impl<'a> FromCommonMessage<'a> for &'a Option<String> {
    fn from(req: &'a dyn CommonMessage) -> Self {
        req.get_content()
    }
}

impl FromCommonMessage<'_> for Option<String> {
    fn from(req: &dyn CommonMessage) -> Self {
        req.get_content().clone()
    }
}

impl FromCommonMessage<'_> for String {
    fn from(req: &dyn CommonMessage) -> Self {
        req.get_content().clone().unwrap_or_default()
    }
}

impl FromCommonMessage<'_> for MessageFrom {
    fn from(req: &dyn CommonMessage) -> Self {
        req.get_message_from_type()
    }
}
