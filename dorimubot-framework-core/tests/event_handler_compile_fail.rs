#[test]
fn event_handlers_reject_payloads_not_produced_by_the_marker() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/event_handler/*.rs");
}
