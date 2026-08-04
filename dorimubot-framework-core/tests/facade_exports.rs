use qqbot_rust_sdk::events::c2c::event::C2cEventKind;
use qqbot_rust_sdk::events::payload::event::EventKind;
use qqbot_rust_sdk::events::payload::payload::{
    DispatchPayload, FromDispatchPayload, WebhookPayload,
};
use qqbot_rust_sdk::openapi::error::Result;
use qqbot_rust_sdk::openapi::models::UploadMediaRequest;
use qqbot_rust_sdk::openapi::{HttpTokenProvider, OpenApi};

#[test]
fn core_sdk_types_are_imported_from_the_sdk() {
    let _: EventKind = C2cEventKind::C2cMessageCreate.into();

    fn accepts_payload(_: Option<DispatchPayload>) {}
    fn accepts_webhook(_: Option<WebhookPayload>) {}
    fn accepts_extractor<T: FromDispatchPayload>() {}
    fn accepts_api(_: Option<OpenApi<HttpTokenProvider>>) {}
    fn accepts_result(_: Result<()>) {}
    fn accepts_model(_: Option<UploadMediaRequest>) {}

    accepts_payload(None);
    accepts_webhook(None);
    accepts_extractor::<DispatchPayload>();
    accepts_api(None);
    accepts_result(Ok(()));
    accepts_model(None);
}
