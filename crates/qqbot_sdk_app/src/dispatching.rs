use crate::app::QQBotApp;
use qqbot_sdk_core::events::event::Event;
use qqbot_sdk_core::events::payload::{DispatchPayload, WebhookPayload};
use qqbot_sdk_core::events::validation::{ValidationRequest, ValidationResponse};
use qqbot_sdk_core::EventKind;
use tracing::debug;

impl QQBotApp {
    /// Webhook 的第一层 opcode 分发。
    pub async fn webhook_handler(&self, payload: WebhookPayload) -> Option<ValidationResponse> {
        debug!("收到Webhook事件: {:?}", payload);
        match payload {
            WebhookPayload::Dispatch(payload) => {
                self.dispatch_event(payload).await;
                None
            }
            WebhookPayload::HttpCallbackAck(_) => None,
            WebhookPayload::WebhookAddressVerify(payload) => {
                Some(self.handle_address_verify(payload.d).unwrap())
            }
        }
    }

    /// 处理腾讯端请求签名校验。
    pub fn handle_address_verify(
        &self,
        req: ValidationRequest,
    ) -> Result<ValidationResponse, Box<dyn std::error::Error>> {
        let signature = qqbot_sdk_core::signature::sign_webhook_validation(
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
            Event::C2cEventType(event) => self.dispatch_kind(event.to_kind(), &payload).await,
            Event::GroupEventType(event) => self.dispatch_kind(event.to_kind(), &payload).await,
            Event::GuildEventType(event) => self.dispatch_kind(event.to_kind(), &payload).await,
            Event::ForumEventType(event) => self.dispatch_kind(event.to_kind(), &payload).await,
            Event::InteractionEventType(event) => {
                self.dispatch_kind(event.to_kind(), &payload).await
            }
            Event::MessageReactionEventType(event) => {
                self.dispatch_kind(event.to_kind(), &payload).await
            }
        }
    }

    async fn dispatch_kind<K>(&self, kind: K, payload: &DispatchPayload)
    where
        K: Into<EventKind>,
    {
        for handler in self.event_handlers.handlers_for(kind) {
            handler(payload, &self.depend_store).await
        }
    }
}
