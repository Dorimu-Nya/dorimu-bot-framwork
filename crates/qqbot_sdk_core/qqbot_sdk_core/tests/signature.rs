use qqbot_sdk_core::sign_webhook_validation;

#[test]
fn verify_signature() {
    let secret = "DG5g3B4j9X2KOErG";
    let plain_token = "Arq0D5A61EgUu4OxUvOp";
    let event_ts = 1725442341.to_string();
    let expect = "87befc99c42c651b3aac0278e71ada338433ae26fcb24307bdc5ad38c1adc2d01bcfcadc0842edac85e85205028a1132afe09280305f13aa6909ffc2d652c706";

    let res = sign_webhook_validation(secret, &event_ts, plain_token);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expect);
}

#[test]
fn verify_signature_empty_secret_returns_error() {
    let res = sign_webhook_validation("", "1725442341", "Arq0D5A61EgUu4OxUvOp");
    assert!(matches!(res, Err(qqbot_sdk_core::Error::InvalidHeader(_))));
}

#[test]
fn verify_signature_changes_when_payload_changes() {
    let secret = "DG5g3B4j9X2KOErG";
    let sig1 = sign_webhook_validation(secret, "1725442341", "Arq0D5A61EgUu4OxUvOp").unwrap();
    let sig2 = sign_webhook_validation(secret, "1725442342", "Arq0D5A61EgUu4OxUvOp").unwrap();
    assert_ne!(sig1, sig2);
}
