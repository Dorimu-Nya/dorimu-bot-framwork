use qqbot_sdk::command;
use qqbot_sdk::events::payload::{DispatchPayload, WebhookPayload};
use qqbot_sdk::{App, AppConfig, CommandPlugin, Depend};
use std::sync::atomic::{AtomicUsize, Ordering};

struct Counter(AtomicUsize);

fn c2c_payload(content: &str) -> DispatchPayload {
    serde_json::from_value(serde_json::json!({
        "id": "event-id",
        "op": 0,
        "s": 1,
        "t": "C2C_MESSAGE_CREATE",
        "d": {
            "id": "message-id",
            "author": { "user_openid": "user-id" },
            "content": content,
            "msg_seq": 1
        }
    }))
    .unwrap()
}

#[command("/depend-test")]
fn injected_command(counter: Depend<Counter>) {
    counter.0.fetch_add(1, Ordering::SeqCst);
}

fn manually_registered_command(counter: Depend<Counter>) {
    counter.0.fetch_add(1, Ordering::SeqCst);
}

#[tokio::test]
async fn manually_loaded_command_plugin_runs_macro_and_manual_commands() {
    let command_plugin =
        CommandPlugin::new().with_command("/manual-depend-test", manually_registered_command);
    let app = App::new(
        AppConfig::new()
            .with_depend(Depend::new(Counter(AtomicUsize::new(0))))
            .with_plugin(command_plugin),
    );

    app.webhook_handler(WebhookPayload::Dispatch(c2c_payload("/manual-depend-test")))
        .await;
    app.webhook_handler(WebhookPayload::Dispatch(c2c_payload("/depend-test")))
        .await;

    assert_eq!(
        app.depend_store.get::<Counter>().0.load(Ordering::SeqCst),
        2
    );
}
