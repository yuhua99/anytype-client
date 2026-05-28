use anyclient::models::*;
use serde_json::json;

#[test]
fn search_request_preserves_legacy_raw_filter_shape() {
    let legacy = json!({
        "type": "and",
        "filters": [
            {"key":"type","condition":"equal","value":"task"},
            {"key":"status","condition":"equal","value":"done"}
        ]
    });

    let req = SearchRequest {
        query: String::new(),
        types: Vec::new(),
        filters: Some(legacy.clone().into()),
        sort: None,
    };

    let body = serde_json::to_value(req).unwrap();

    assert_eq!(body["filters"], legacy);
}

#[test]
fn search_request_preserves_unknown_legacy_filter_fields() {
    let legacy = json!({
        "type": "or",
        "filters": [{"key":"custom","operator":"legacy-op","value":{"nested":true}}],
        "vendor_extension": {"keep":"me"}
    });

    let req = SearchRequest {
        query: "legacy".into(),
        types: vec!["page".into()],
        filters: Some(legacy.clone().into()),
        sort: None,
    };

    let body = serde_json::to_value(req).unwrap();

    assert_eq!(body["filters"], legacy);
    assert_eq!(body["filters"]["vendor_extension"]["keep"], "me");
}
