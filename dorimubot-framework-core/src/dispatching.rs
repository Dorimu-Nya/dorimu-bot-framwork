use crate::app::QQBot;
use crate::event_handler::EventHandlerInput;
use qqbot_rust_sdk::events::c2c::event::{C2cEvent, C2cEventKind};
use qqbot_rust_sdk::events::group::event::{GroupEvent, GroupEventKind};
use qqbot_rust_sdk::events::guild::event::{
    ForumEvent, ForumEventKind, GuildEvent, GuildEventKind,
};
use qqbot_rust_sdk::events::interaction::event::{InteractionEvent, InteractionEventKind};
use qqbot_rust_sdk::events::message_reaction::event::{
    MessageReactionEvent, MessageReactionEventKind,
};
use qqbot_rust_sdk::events::payload::event::{Event, EventKind};
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use qqbot_rust_sdk::events::validation::{ValidationRequest, ValidationResponse};
use std::any::Any;
use tracing::debug;

/// 提供事件枚举当前变体实际携带的值。
///
/// 这里使用穷尽匹配，以便 SDK 新增事件变体时在编译期提醒框架补充映射。
trait DispatchEvent {
    type Kind: Into<EventKind>;

    fn kind(&self) -> Self::Kind;
    fn value(&self) -> &(dyn Any + Send + Sync);
}

impl DispatchEvent for C2cEvent {
    type Kind = C2cEventKind;

    fn kind(&self) -> Self::Kind {
        self.to_kind()
    }

    fn value(&self) -> &(dyn Any + Send + Sync) {
        match self {
            Self::C2cMessageCreate(value) => value,
            Self::FriendAdd(value) => value,
            Self::FriendDel(value) => value,
            Self::C2cMsgReject(value) => value,
            Self::C2cMsgReceive(value) => value,
        }
    }
}

impl DispatchEvent for GroupEvent {
    type Kind = GroupEventKind;

    fn kind(&self) -> Self::Kind {
        self.to_kind()
    }

    fn value(&self) -> &(dyn Any + Send + Sync) {
        match self {
            Self::GroupAtMessageCreate(value) | Self::GroupMessageCreate(value) => value,
            Self::GroupAddRobot(value) | Self::GroupMsgReceive(value) => value,
            Self::GroupDelRobot(value) => value,
            Self::GroupMsgReject(value) => value,
            Self::SubscribeMessageStatus => &(),
        }
    }
}

impl DispatchEvent for GuildEvent {
    type Kind = GuildEventKind;

    fn kind(&self) -> Self::Kind {
        self.to_kind()
    }

    fn value(&self) -> &(dyn Any + Send + Sync) {
        match self {
            Self::AtMessageCreate(value) | Self::DirectMessageCreate(value) => value,
            Self::PublicMessageDelete()
            | Self::DirectMessageDelete()
            | Self::MessageReactionAdd
            | Self::MessageReactionRemove
            | Self::MessageAuditPass()
            | Self::MessageAuditReject()
            | Self::AudioStart()
            | Self::AudioFinish()
            | Self::AudioOnMic()
            | Self::AudioOffMic() => &(),
            Self::OpenForumThreadCreate(value)
            | Self::OpenForumPostCreate(value)
            | Self::OpenForumReplyCreate(value)
            | Self::OpenForumThreadUpdate(value)
            | Self::OpenForumPostDelete(value)
            | Self::OpenForumReplyDelete(value)
            | Self::OpenForumThreadDelete(value) => value,
            Self::GuildCreate(value) | Self::GuildUpdate(value) | Self::GuildDelete(value) => value,
            Self::ChannelCreate(value)
            | Self::ChannelUpdate(value)
            | Self::ChannelDelete(value) => value,
            Self::GuildMemberAdd(value)
            | Self::GuildMemberRemove(value)
            | Self::GuildMemberUpdate(value) => value,
            Self::AudioOrLiveChannelMemberEnter(value)
            | Self::AudioOrLiveChannelMemberExit(value) => value,
        }
    }
}

impl DispatchEvent for ForumEvent {
    type Kind = ForumEventKind;

    fn kind(&self) -> Self::Kind {
        self.to_kind()
    }

    fn value(&self) -> &(dyn Any + Send + Sync) {
        match self {
            Self::ForumThreadCreate(value)
            | Self::ForumThreadUpdate(value)
            | Self::ForumThreadDelete(value) => value,
            Self::ForumPostCreate(value) | Self::ForumPostDelete(value) => value,
            Self::ForumReplyCreate(value) | Self::ForumReplyDelete(value) => value,
            Self::ForumAuditEvent(value) => value,
        }
    }
}

impl DispatchEvent for InteractionEvent {
    type Kind = InteractionEventKind;

    fn kind(&self) -> Self::Kind {
        self.to_kind()
    }

    fn value(&self) -> &(dyn Any + Send + Sync) {
        match self {
            Self::InteractionCreate(value) => value,
        }
    }
}

impl DispatchEvent for MessageReactionEvent {
    type Kind = MessageReactionEventKind;

    fn kind(&self) -> Self::Kind {
        self.to_kind()
    }

    fn value(&self) -> &(dyn Any + Send + Sync) {
        match self {
            Self::MessageReactionAdd(value) | Self::MessageReactionRemove(value) => value,
        }
    }
}

impl QQBot {
    /// Webhook 的第一层 opcode 分发。
    pub async fn webhook_handler(&self, payload: WebhookPayload) -> Option<ValidationResponse> {
        debug!("收到Webhook事件: {:?}", payload);
        match payload {
            WebhookPayload::Dispatch(payload) => {
                self.dispatch_event(payload).await;
                None
            }
            WebhookPayload::WebhookAddressVerify(payload) => {
                Some(self.handle_address_verify(payload.d).unwrap())
            }

            _ => None,
        }
    }

    /// 处理腾讯端请求签名校验。
    fn handle_address_verify(
        &self,
        req: ValidationRequest,
    ) -> Result<ValidationResponse, Box<dyn std::error::Error>> {
        let signature = qqbot_rust_sdk::signature::sign_webhook_validation(
            &self.credential.secret,
            &req.event_ts,
            &req.plain_token,
        )?;
        Ok(ValidationResponse {
            plain_token: req.plain_token,
            signature,
        })
    }

    /// 处理 opcode 为 0 的事件分发。
    async fn dispatch_event(&self, payload: DispatchPayload) {
        match &payload.event {
            Event::C2cEvent(event) => self.dispatch_typed_event(event, &payload).await,
            Event::GroupEvent(event) => self.dispatch_typed_event(event, &payload).await,
            Event::GuildEvent(event) => self.dispatch_typed_event(event, &payload).await,
            Event::ForumEvent(event) => self.dispatch_typed_event(event, &payload).await,
            Event::InteractionEvent(event) => self.dispatch_typed_event(event, &payload).await,
            Event::MessageReactionEvent(event) => self.dispatch_typed_event(event, &payload).await,
        }
    }

    async fn dispatch_typed_event<E>(&self, event: &E, payload: &DispatchPayload)
    where
        E: DispatchEvent,
    {
        let input = EventHandlerInput::new(payload, event.value());
        for handler in self.event_handlers.get_handlers(event.kind()) {
            handler(input).await
        }
    }
}
