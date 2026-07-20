use crate::{AppConfig, QQBotApp};
use qqbot_sdk_core::events::c2c::event_type::C2cEventTypeKind;
use qqbot_sdk_core::events::c2c::models::C2cMessage;
use qqbot_sdk_core::events::payload::DispatchPayload;

#[test]
fn registers_event_handlers_with_supported_signatures() {
    let app = QQBotApp::new(AppConfig::default());

    app.registe_event_handler(
        C2cEventTypeKind::C2cMessageCreate,
        handler_without_arguments,
    );
    app.registe_event_handler(C2cEventTypeKind::C2cMessageCreate, handler_with_payload);
    app.registe_event_handler(
        C2cEventTypeKind::C2cMessageCreate,
        handler_with_event_detail,
    );

    assert_eq!(
        app.event_handlers
            .handlers_for(C2cEventTypeKind::C2cMessageCreate)
            .len(),
        3
    );
}

fn handler_without_arguments() {}

fn handler_with_payload(_payload: &DispatchPayload) {}

fn handler_with_event_detail(_detail: &C2cMessage) {}
