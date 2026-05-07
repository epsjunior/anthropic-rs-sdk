//! Live integration test that hits the real Anthropic API.
//!
//! Gated behind the `live-tests` feature so default `cargo test` (and CI
//! without secrets) never reaches the network. Run with:
//! ```text
//! ANTHROPIC_API_KEY=sk-ant-… cargo test --features live-tests
//! ```

#![cfg(feature = "live-tests")]

use anthropic::types::{InputMessage, MessageCreateParams, model};

#[tokio::test]
async fn live_smoke_haiku_replies() {
    let client = anthropic::Client::from_env().expect("ANTHROPIC_API_KEY must be set");
    let response = client
        .messages()
        .create(
            MessageCreateParams::builder()
                .model(model::CLAUDE_HAIKU_4_5)
                .max_tokens(64)
                .messages(vec![InputMessage::user("Reply with the single word: ok")])
                .build(),
        )
        .await
        .expect("messages.create should succeed against the live API");

    assert!(!response.text().is_empty(), "expected non-empty reply");
    assert!(
        response.usage.output_tokens > 0,
        "expected non-zero output tokens"
    );
}
