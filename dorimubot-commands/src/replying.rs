use qqbot_rust_sdk::openapi::apis::message::models::message_type::MessageType;
use qqbot_rust_sdk::openapi::apis::message::models::{
    message_ark::MessageArk, message_embed::MessageEmbed, message_markdown::MessageMarkdown,
    message_media::MessageMedia, send_message_request::SendMessageRequest,
};

/// 指示回复的会话类型：私聊（C2c）或群组（Group）。
pub enum ReplyingType {
    /// 一对一私聊。
    C2c,
    /// 群组聊天。
    Group,
}

/// 回复消息的类型
///
/// 变体：
/// - `Text`：纯文本消息
/// - `Markdown`：Markdown 模式消息
/// - `Ark`：Ark 模板消息
/// - `Embed`：嵌入卡片消息
/// - `Media`：媒体消息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ReplyingMessage {
    /// 纯文本消息。
    Text(String),
    /// Markdown 模板消息。
    Markdown(MessageMarkdown),
    /// Ark 模板消息。
    Ark(MessageArk),
    /// 嵌入卡片消息。
    Embed(MessageEmbed),
    /// 媒体消息。
    Media(MessageMedia),
}

impl ReplyingMessage {
    /// 将枚举映射到msg_type的数值。
    pub fn to_msg_type(&self) -> MessageType {
        match self {
            Self::Text(_) => MessageType::Text,
            Self::Markdown(_) => MessageType::Markdown,
            Self::Ark(_) => MessageType::Ark,
            Self::Embed(_) => MessageType::Embed,
            Self::Media(_) => MessageType::Media,
        }
    }

    pub fn to_request(&self, msg_id: Option<String>, msg_seq: Option<u64>) -> SendMessageRequest {
        let basic = SendMessageRequest {
            msg_id,
            msg_seq,
            msg_type: self.to_msg_type(),
            content: None,
            markdown: None,
            keyboard: None,
            ark: None,
            media: None,
            embed: None,
            message_reference: None,
            event_id: None,
        };
        match self {
            Self::Text(text) => SendMessageRequest {
                content: Some(text.clone()),
                ..basic
            },
            Self::Markdown(markdown) => SendMessageRequest {
                markdown: Some(markdown.clone()),
                keyboard: markdown.keyboard.clone(),
                ..basic
            },
            Self::Ark(ark) => SendMessageRequest {
                ark: Some(ark.clone()),
                ..basic
            },
            Self::Embed(embed) => SendMessageRequest {
                embed: Some(embed.clone()),
                ..basic
            },
            Self::Media(media) => SendMessageRequest {
                media: Some(media.clone()),
                ..basic
            },
        }
    }
}
