use qqbot_sdk_runtime::DependStore;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Counter(AtomicUsize);

#[test]
fn depend_store_resolves_values_and_arcs() {
    let store = DependStore::new();
    store.insert(Counter(AtomicUsize::new(1)));
    assert_eq!(store.get::<Counter>().0.load(Ordering::SeqCst), 1);

    let replacement = std::sync::Arc::new(Counter(AtomicUsize::new(2)));
    assert!(store.insert_arc(replacement).is_some());
    assert_eq!(store.get_depend::<Counter>().0.load(Ordering::SeqCst), 2);
}

#[test]
#[should_panic(expected = "dependency not found")]
fn missing_dependency_panics_when_resolved() {
    DependStore::new().get::<Counter>();
}
