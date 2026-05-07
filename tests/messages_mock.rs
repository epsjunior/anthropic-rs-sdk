//! End-to-end test against an in-process `wiremock` server.
//!
//! No `ANTHROPIC_API_KEY` required — the mock validates the request shape
//! the SDK sends and returns canned responses to exercise both the happy
//! path and the typed error path.

use anthropic::types::{InputMessage, MessageCreateParams, model};
use anthropic::{Client, Error, ErrorType};
use serde_json::{Value, json};
use url::Url;
use wiremock::matchers::{header, header_exists, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

async fn build_client(server: &MockServer) -> Client {
    Client::builder()
        .api_key("sk-ant-test")
        .base_url(Url::parse(&server.uri()).unwrap())
        .build()
        .expect("client")
}

#[tokio::test]
async fn create_sends_expected_request_and_decodes_response() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .and(header("x-api-key", "sk-ant-test"))
        .and(header("anthropic-version", "2023-06-01"))
        .and(header("content-type", "application/json"))
        .and(header_exists("user-agent"))
        .respond_with(|req: &Request| {
            // Validate the request body shape once it actually arrives.
            let body: Value = serde_json::from_slice(&req.body).expect("body is JSON");
            assert_eq!(body["model"], json!("claude-haiku-4-5"));
            assert_eq!(body["max_tokens"], json!(64));
            assert_eq!(
                body["messages"],
                json!([{"role":"user","content":"hello mock"}])
            );

            ResponseTemplate::new(200)
                .insert_header("request-id", "req_test_123")
                .set_body_json(json!({
                    "id": "msg_01TEST",
                    "type": "message",
                    "role": "assistant",
                    "content": [{"type":"text","text":"hi from mock"}],
                    "model": "claude-haiku-4-5",
                    "stop_reason": "end_turn",
                    "stop_sequence": null,
                    "usage": {"input_tokens": 8, "output_tokens": 4}
                }))
        })
        .expect(1)
        .mount(&server)
        .await;

    let client = build_client(&server).await;
    let response = client
        .messages()
        .create(
            MessageCreateParams::builder()
                .model(model::CLAUDE_HAIKU_4_5)
                .max_tokens(64)
                .messages(vec![InputMessage::user("hello mock")])
                .build(),
        )
        .await
        .expect("happy-path call should succeed");

    assert_eq!(response.id, "msg_01TEST");
    assert_eq!(response.text(), "hi from mock");
    assert_eq!(response.usage.output_tokens, 4);
}

#[tokio::test]
async fn non_2xx_response_becomes_typed_api_error_with_request_id() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .respond_with(
            ResponseTemplate::new(401)
                .insert_header("request-id", "req_unauth_42")
                .set_body_json(json!({
                    "type": "error",
                    "error": {
                        "type": "authentication_error",
                        "message": "invalid x-api-key"
                    }
                })),
        )
        .mount(&server)
        .await;

    let client = build_client(&server).await;
    let err = client
        .messages()
        .create(
            MessageCreateParams::builder()
                .model(model::CLAUDE_HAIKU_4_5)
                .max_tokens(8)
                .messages(vec![InputMessage::user("…")])
                .build(),
        )
        .await
        .expect_err("401 must surface as Error::Api");

    match err {
        Error::Api {
            status,
            error_type,
            message,
            request_id,
            ..
        } => {
            assert_eq!(status, http::StatusCode::UNAUTHORIZED);
            assert_eq!(error_type, Some(ErrorType::AuthenticationError));
            assert_eq!(message, "invalid x-api-key");
            assert_eq!(request_id.as_deref(), Some("req_unauth_42"));
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn non_json_error_body_still_yields_api_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .respond_with(ResponseTemplate::new(503).set_body_string("upstream is on fire"))
        .mount(&server)
        .await;

    let client = build_client(&server).await;
    let err = client
        .messages()
        .create(
            MessageCreateParams::builder()
                .model(model::CLAUDE_HAIKU_4_5)
                .max_tokens(8)
                .messages(vec![InputMessage::user("…")])
                .build(),
        )
        .await
        .expect_err("non-JSON 503 must still be Error::Api");

    match err {
        Error::Api {
            status,
            error_type,
            raw,
            ..
        } => {
            assert_eq!(status, http::StatusCode::SERVICE_UNAVAILABLE);
            assert!(error_type.is_none());
            assert_eq!(raw, "upstream is on fire");
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[test]
fn from_env_errors_when_api_key_missing() {
    // SAFETY of test: this temporarily clears the env var; if other tests
    // in the same process rely on it they'd be racy. We restore it after.
    let prior = std::env::var("ANTHROPIC_API_KEY").ok();
    // SAFETY: setting/unsetting env vars in tests is not thread-safe in
    // general, but cargo test isolates each #[test] in its own task and
    // we restore the prior value before returning.
    unsafe {
        std::env::remove_var("ANTHROPIC_API_KEY");
    }

    let result = Client::from_env();
    assert!(matches!(result, Err(Error::Config(_))));

    if let Some(v) = prior {
        unsafe {
            std::env::set_var("ANTHROPIC_API_KEY", v);
        }
    }
}
