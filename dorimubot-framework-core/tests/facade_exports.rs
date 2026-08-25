use qqbot_rust_sdk::events::c2c::event::C2cEventKind;
use qqbot_rust_sdk::events::payload::event::EventKind;
use qqbot_rust_sdk::events::payload::payload::{DispatchPayload, WebhookPayload};
use qqbot_rust_sdk::openapi::api::QQApiClient;
use qqbot_rust_sdk::openapi::api_request::ApiRequestError;
use qqbot_rust_sdk::openapi::apis::message::media::models::upload_media_request::UploadMediaRequest;
use qqbot_rust_sdk::openapi::models::err_resp::ErrResp;

#[test]
fn core_sdk_types_are_imported_from_the_sdk() {
    let _: EventKind = C2cEventKind::C2cMessageCreate.into();

    fn accepts_payload(_: Option<DispatchPayload>) {}
    fn accepts_webhook(_: Option<WebhookPayload>) {}
    fn accepts_api(_: Option<QQApiClient>) {}
    fn accepts_result(_: Result<(), ApiRequestError<ErrResp>>) {}
    fn accepts_model(_: Option<UploadMediaRequest>) {}

    accepts_payload(None);
    accepts_webhook(None);
    accepts_api(None);
    accepts_result(Ok(()));
    accepts_model(None);
}
