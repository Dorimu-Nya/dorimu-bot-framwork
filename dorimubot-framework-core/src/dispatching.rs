use crate::app::QQBot;
use qqbot_rust_sdk::events::payload::event::Event;
use qqbot_rust_sdk::events::payload::event::EventKind;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use qqbot_rust_sdk::events::validation::{ValidationRequest, ValidationResponse};
use std::any::Any;
use tracing::debug;

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
        let (kind, event_data): (EventKind, &(dyn Any + Send + Sync)) = match &payload.event {
            Event::C2cEvent(event) => (event.to_kind().into(), event.data()),
            Event::GroupEvent(event) => (event.to_kind().into(), event.data()),
            Event::GuildEvent(event) => (event.to_kind().into(), event.data()),
            Event::ForumEvent(event) => (event.to_kind().into(), event.data()),
            Event::InteractionEvent(event) => (event.to_kind().into(), event.data()),
            Event::MessageReactionEvent(event) => (event.to_kind().into(), event.data()),
        };

        self.dispatch_kind(kind, event_data).await;
    }

    async fn dispatch_kind(&self, kind: EventKind, event_data: &(dyn Any + Send + Sync)) {
        for handler in self.event_handlers.get_handlers(kind) {
            handler(event_data).await
        }
    }
}
