use qqbot_sdk::events::c2c::models::C2cMessage;
use qqbot_sdk::events::payload::DispatchPayload;
use qqbot_sdk::{Depend, DependStore, DynEventHandler, EventHandler};
use std::sync::atomic::{AtomicUsize, Ordering};

struct Counter(AtomicUsize);

fn into_event_handler<H, Args, Kind>(handler: H) -> DynEventHandler
where
    H: EventHandler<Args, Kind>,
{
    handler.into_dyn()
}

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

#[tokio::test]
async fn event_handlers_mix_payload_and_dependencies() {
    let store = DependStore::new();
    store.insert(Counter(AtomicUsize::new(0)));

    let handler = into_event_handler(
        |message: C2cMessage, counter: Depend<Counter>, dependencies: DependStore| async move {
            assert_eq!(message.content.as_deref(), Some("hello"));
            assert_eq!(dependencies.get::<Counter>().0.load(Ordering::SeqCst), 0);
            counter.0.fetch_add(1, Ordering::SeqCst);
        },
    );

    handler(&c2c_payload("hello"), &store).await;
    assert_eq!(store.get::<Counter>().0.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn synchronous_owned_event_handlers_support_dependencies() {
    let store = DependStore::new();
    store.insert(Counter(AtomicUsize::new(0)));
    let handler = into_event_handler(|_message: C2cMessage, counter: Depend<Counter>| {
        counter.0.fetch_add(1, Ordering::SeqCst);
    });

    handler(&c2c_payload("hello"), &store).await;
    assert_eq!(store.get::<Counter>().0.load(Ordering::SeqCst), 1);
}
