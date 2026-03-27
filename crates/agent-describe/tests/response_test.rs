use agent_describe::AgentResponse;
use serde_json::Value;

#[test]
fn ok_response_serializes_correctly() {
    #[derive(serde::Serialize)]
    struct MyResult {
        url: String,
    }

    let resp = AgentResponse::ok(MyResult {
        url: "https://example.com".into(),
    });
    let json: Value = serde_json::from_str(&resp.to_json()).unwrap();

    assert_eq!(json["ok"], true);
    assert_eq!(json["data"]["url"], "https://example.com");
    assert!(json.get("error").is_none());
}

#[test]
fn err_response_serializes_correctly() {
    let resp = AgentResponse::<()>::err("not found", Some("try `mycli list`"));
    let json: Value = serde_json::from_str(&resp.to_json()).unwrap();

    assert_eq!(json["ok"], false);
    assert_eq!(json["error"], "not found");
    assert_eq!(json["suggestion"], "try `mycli list`");
}

#[test]
fn err_response_without_suggestion() {
    let resp = AgentResponse::<()>::err("failed", None::<String>);
    let json: Value = serde_json::from_str(&resp.to_json()).unwrap();

    assert_eq!(json["ok"], false);
    assert_eq!(json["error"], "failed");
    assert!(json["suggestion"].is_null());
}
