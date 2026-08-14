use crate::app::QQBot;
use crate::events::{self, EventSpec};
use qqbot_rust_sdk::events::c2c::event::C2cEvent;
use qqbot_rust_sdk::events::group::event::GroupEvent;
use qqbot_rust_sdk::events::guild::event::{ForumEvent, GuildEvent};
use qqbot_rust_sdk::events::interaction::event::InteractionEvent;
use qqbot_rust_sdk::events::message_reaction::event::MessageReactionEvent;
use qqbot_rust_sdk::events::payload::event::Event;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use qqbot_rust_sdk::events::validation::{ValidationRequest, ValidationResponse};
use std::any::Any;
use tracing::debug;

/// 无载荷事件分发时共享的单元值。
static EMPTY_PAYLOAD: () = ();

impl QQBot {
    /// Webhook 的第一层操作码分发。
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

    fn handle_address_verify(
        &self,
        request: ValidationRequest,
    ) -> Result<ValidationResponse, Box<dyn std::error::Error>> {
        let signature = qqbot_rust_sdk::signature::sign_webhook_validation(
            &self.credential.secret,
            &request.event_ts,
            &request.plain_token,
        )?;
        Ok(ValidationResponse {
            plain_token: request.plain_token,
            signature,
        })
    }

    async fn dispatch_event(&self, payload: DispatchPayload) {
        match &payload.event {
            Event::C2cEvent(event) => self.dispatch_c2c(event).await,
            Event::GroupEvent(event) => self.dispatch_group(event).await,
            Event::GuildEvent(event) => self.dispatch_guild(event).await,
            Event::ForumEvent(event) => self.dispatch_forum(event).await,
            Event::InteractionEvent(event) => self.dispatch_interaction(event).await,
            Event::MessageReactionEvent(event) => self.dispatch_message_reaction(event).await,
        }
    }

    async fn dispatch_c2c(&self, event: &C2cEvent) {
        match event {
            C2cEvent::C2cMessageCreate(payload) => {
                self.dispatch::<events::c2c::C2cMessageCreate>(payload)
                    .await
            }
            C2cEvent::FriendAdd(payload) => self.dispatch::<events::c2c::FriendAdd>(payload).await,
            C2cEvent::FriendDel(payload) => self.dispatch::<events::c2c::FriendDel>(payload).await,
            C2cEvent::C2cMsgReject(payload) => {
                self.dispatch::<events::c2c::C2cMsgReject>(payload).await
            }
            C2cEvent::C2cMsgReceive(payload) => {
                self.dispatch::<events::c2c::C2cMsgReceive>(payload).await
            }
        }
    }

    async fn dispatch_group(&self, event: &GroupEvent) {
        match event {
            GroupEvent::GroupAtMessageCreate(payload) => {
                self.dispatch::<events::group::GroupAtMessageCreate>(payload)
                    .await
            }
            GroupEvent::GroupMessageCreate(payload) => {
                self.dispatch::<events::group::GroupMessageCreate>(payload)
                    .await
            }
            GroupEvent::GroupAddRobot(payload) => {
                self.dispatch::<events::group::GroupAddRobot>(payload).await
            }
            GroupEvent::GroupDelRobot(payload) => {
                self.dispatch::<events::group::GroupDelRobot>(payload).await
            }
            GroupEvent::GroupMsgReceive(payload) => {
                self.dispatch::<events::group::GroupMsgReceive>(payload)
                    .await
            }
            GroupEvent::GroupMsgReject(payload) => {
                self.dispatch::<events::group::GroupMsgReject>(payload)
                    .await
            }
            GroupEvent::SubscribeMessageStatus => {
                self.dispatch::<events::group::SubscribeMessageStatus>(&EMPTY_PAYLOAD)
                    .await
            }
        }
    }

    async fn dispatch_guild(&self, event: &GuildEvent) {
        match event {
            GuildEvent::AtMessageCreate(payload) => {
                self.dispatch::<events::guild::AtMessageCreate>(payload)
                    .await
            }
            GuildEvent::PublicMessageDelete() => {
                self.dispatch::<events::guild::PublicMessageDelete>(&EMPTY_PAYLOAD)
                    .await
            }
            GuildEvent::DirectMessageCreate(payload) => {
                self.dispatch::<events::guild::DirectMessageCreate>(payload)
                    .await
            }
            GuildEvent::DirectMessageDelete() => {
                self.dispatch::<events::guild::DirectMessageDelete>(&EMPTY_PAYLOAD)
                    .await
            }
            GuildEvent::MessageReactionAdd => {
                self.dispatch::<events::guild::MessageReactionAdd>(&EMPTY_PAYLOAD)
                    .await
            }
            GuildEvent::MessageReactionRemove => {
                self.dispatch::<events::guild::MessageReactionRemove>(&EMPTY_PAYLOAD)
                    .await
            }
            GuildEvent::MessageAuditPass() => {
                self.dispatch::<events::guild::MessageAuditPass>(&EMPTY_PAYLOAD)
                    .await
            }
            GuildEvent::MessageAuditReject() => {
                self.dispatch::<events::guild::MessageAuditReject>(&EMPTY_PAYLOAD)
                    .await
            }
            GuildEvent::OpenForumThreadCreate(payload) => {
                self.dispatch::<events::guild::OpenForumThreadCreate>(payload)
                    .await
            }
            GuildEvent::OpenForumPostCreate(payload) => {
                self.dispatch::<events::guild::OpenForumPostCreate>(payload)
                    .await
            }
            GuildEvent::OpenForumReplyCreate(payload) => {
                self.dispatch::<events::guild::OpenForumReplyCreate>(payload)
                    .await
            }
            GuildEvent::OpenForumThreadUpdate(payload) => {
                self.dispatch::<events::guild::OpenForumThreadUpdate>(payload)
                    .await
            }
            GuildEvent::OpenForumPostDelete(payload) => {
                self.dispatch::<events::guild::OpenForumPostDelete>(payload)
                    .await
            }
            GuildEvent::OpenForumReplyDelete(payload) => {
                self.dispatch::<events::guild::OpenForumReplyDelete>(payload)
                    .await
            }
            GuildEvent::OpenForumThreadDelete(payload) => {
                self.dispatch::<events::guild::OpenForumThreadDelete>(payload)
                    .await
            }
            GuildEvent::GuildCreate(payload) => {
                self.dispatch::<events::guild::GuildCreate>(payload).await
            }
            GuildEvent::GuildUpdate(payload) => {
                self.dispatch::<events::guild::GuildUpdate>(payload).await
            }
            GuildEvent::GuildDelete(payload) => {
                self.dispatch::<events::guild::GuildDelete>(payload).await
            }
            GuildEvent::ChannelCreate(payload) => {
                self.dispatch::<events::guild::ChannelCreate>(payload).await
            }
            GuildEvent::ChannelUpdate(payload) => {
                self.dispatch::<events::guild::ChannelUpdate>(payload).await
            }
            GuildEvent::ChannelDelete(payload) => {
                self.dispatch::<events::guild::ChannelDelete>(payload).await
            }
            GuildEvent::GuildMemberAdd(payload) => {
                self.dispatch::<events::guild::GuildMemberAdd>(payload)
                    .await
            }
            GuildEvent::GuildMemberRemove(payload) => {
                self.dispatch::<events::guild::GuildMemberRemove>(payload)
                    .await
            }
            GuildEvent::GuildMemberUpdate(payload) => {
                self.dispatch::<events::guild::GuildMemberUpdate>(payload)
                    .await
            }
            GuildEvent::AudioStart() => {
                self.dispatch::<events::guild::AudioStart>(&EMPTY_PAYLOAD)
                    .await
            }
            GuildEvent::AudioFinish() => {
                self.dispatch::<events::guild::AudioFinish>(&EMPTY_PAYLOAD)
                    .await
            }
            GuildEvent::AudioOnMic() => {
                self.dispatch::<events::guild::AudioOnMic>(&EMPTY_PAYLOAD)
                    .await
            }
            GuildEvent::AudioOffMic() => {
                self.dispatch::<events::guild::AudioOffMic>(&EMPTY_PAYLOAD)
                    .await
            }
            GuildEvent::AudioOrLiveChannelMemberEnter(payload) => {
                self.dispatch::<events::guild::AudioOrLiveChannelMemberEnter>(payload)
                    .await
            }
            GuildEvent::AudioOrLiveChannelMemberExit(payload) => {
                self.dispatch::<events::guild::AudioOrLiveChannelMemberExit>(payload)
                    .await
            }
        }
    }

    async fn dispatch_forum(&self, event: &ForumEvent) {
        match event {
            ForumEvent::ForumThreadCreate(payload) => {
                self.dispatch::<events::forum::ForumThreadCreate>(payload)
                    .await
            }
            ForumEvent::ForumThreadUpdate(payload) => {
                self.dispatch::<events::forum::ForumThreadUpdate>(payload)
                    .await
            }
            ForumEvent::ForumThreadDelete(payload) => {
                self.dispatch::<events::forum::ForumThreadDelete>(payload)
                    .await
            }
            ForumEvent::ForumPostCreate(payload) => {
                self.dispatch::<events::forum::ForumPostCreate>(payload)
                    .await
            }
            ForumEvent::ForumPostDelete(payload) => {
                self.dispatch::<events::forum::ForumPostDelete>(payload)
                    .await
            }
            ForumEvent::ForumReplyCreate(payload) => {
                self.dispatch::<events::forum::ForumReplyCreate>(payload)
                    .await
            }
            ForumEvent::ForumReplyDelete(payload) => {
                self.dispatch::<events::forum::ForumReplyDelete>(payload)
                    .await
            }
            ForumEvent::ForumAuditEvent(payload) => {
                self.dispatch::<events::forum::ForumAuditEvent>(payload)
                    .await
            }
        }
    }

    async fn dispatch_interaction(&self, event: &InteractionEvent) {
        match event {
            InteractionEvent::InteractionCreate(payload) => {
                self.dispatch::<events::interaction::InteractionCreate>(payload)
                    .await
            }
        }
    }

    async fn dispatch_message_reaction(&self, event: &MessageReactionEvent) {
        match event {
            MessageReactionEvent::MessageReactionAdd(payload) => {
                self.dispatch::<events::message_reaction::MessageReactionAdd>(payload)
                    .await
            }
            MessageReactionEvent::MessageReactionRemove(payload) => {
                self.dispatch::<events::message_reaction::MessageReactionRemove>(payload)
                    .await
            }
        }
    }

    async fn dispatch<E>(&self, payload: &E::Payload)
    where
        E: EventSpec,
    {
        let erased_payload: &(dyn Any + Send + Sync) = payload;
        for handler in self.event_handlers.handlers_for::<E>() {
            handler(erased_payload).await;
        }
    }
}
