use dorimubot_framework::events::c2c::event::C2cEventKind;
use dorimubot_framework::models::UploadMediaRequest;
use dorimubot_framework::{
    DispatchPayload, EventKind, FromDispatchPayload, HttpTokenProvider, OpenApi, Result,
    WebhookPayload,
};

#[test]
fn core_sdk_types_remain_available_from_the_facade() {
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
