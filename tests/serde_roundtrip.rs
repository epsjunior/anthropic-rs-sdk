//! Serde round-trip coverage for every public type the SDK serializes/deserializes.
//!
//! These tests intentionally use the *type names a user would import* (i.e.
//! through the crate's public re-exports) so a refactor that breaks the
//! public surface fails the test suite.

use anthropic::types::{
    ApiErrorBody, ContentBlock, ImageSource, InputMessage, Message, MessageContent,
    MessageCreateParams, Metadata, Role, StopReason, SystemPrompt, ToolResultContent, Usage, model,
};
use pretty_assertions::assert_eq;
use serde_json::{Value, json};

fn roundtrip<T>(value: &T)
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + std::fmt::Debug + PartialEq,
{
    let json_text = serde_json::to_string(value).expect("serialize");
    let restored: T = serde_json::from_str(&json_text).expect("deserialize");
    assert_eq!(*value, restored, "round-trip mismatch");
}

#[test]
fn role_roundtrip() {
    roundtrip(&Role::User);
    roundtrip(&Role::Assistant);
    assert_eq!(serde_json::to_value(Role::User).unwrap(), json!("user"));
    assert_eq!(
        serde_json::to_value(Role::Assistant).unwrap(),
        json!("assistant")
    );
}

#[test]
fn stop_reason_known_and_unknown() {
    for reason in [
        StopReason::EndTurn,
        StopReason::MaxTokens,
        StopReason::StopSequence,
        StopReason::ToolUse,
        StopReason::PauseTurn,
        StopReason::Refusal,
    ] {
        roundtrip(&reason);
    }
    let unknown: StopReason = serde_json::from_str("\"new_reason_invented_tomorrow\"").unwrap();
    assert_eq!(unknown, StopReason::Other);
}

#[test]
fn usage_roundtrips_with_optional_cache_fields() {
    let basic = Usage {
        input_tokens: 12,
        output_tokens: 34,
        ..Usage::default()
    };
    roundtrip(&basic);
    let with_cache = Usage {
        input_tokens: 5,
        output_tokens: 9,
        cache_creation_input_tokens: Some(100),
        cache_read_input_tokens: Some(200),
        service_tier: Some("standard".into()),
    };
    roundtrip(&with_cache);
}

#[test]
fn content_block_text() {
    let block = ContentBlock::text("hello");
    let v = serde_json::to_value(&block).unwrap();
    assert_eq!(v, json!({"type": "text", "text": "hello"}));
    roundtrip(&block);
}

#[test]
fn content_block_image_sources() {
    for source in [
        ImageSource::Base64 {
            media_type: "image/png".into(),
            data: "iVBORw0KGgo=".into(),
        },
        ImageSource::Url {
            url: "https://example.com/cat.png".into(),
        },
        ImageSource::File {
            file_id: "file_abc123".into(),
        },
    ] {
        roundtrip(&ContentBlock::Image { source });
    }
}

#[test]
fn content_block_tool_use_and_result() {
    let tu = ContentBlock::ToolUse {
        id: "toolu_01".into(),
        name: "get_weather".into(),
        input: json!({"city": "Salvador"}),
    };
    roundtrip(&tu);

    let tr_text = ContentBlock::ToolResult {
        tool_use_id: "toolu_01".into(),
        content: ToolResultContent::Text("Sunny, 28°C".into()),
        is_error: None,
    };
    roundtrip(&tr_text);

    let tr_blocks = ContentBlock::ToolResult {
        tool_use_id: "toolu_02".into(),
        content: ToolResultContent::Blocks(vec![ContentBlock::text("multi-block")]),
        is_error: Some(true),
    };
    roundtrip(&tr_blocks);
}

#[test]
fn content_block_thinking_variants() {
    roundtrip(&ContentBlock::Thinking {
        thinking: "Let me consider…".into(),
        signature: "sig_xyz".into(),
    });
    roundtrip(&ContentBlock::RedactedThinking {
        data: "encrypted".into(),
    });
}

#[test]
fn content_block_unknown_forward_compat() {
    let json_text = r#"{"type": "video_clip_v9", "url": "https://x"}"#;
    let block: ContentBlock = serde_json::from_str(json_text).unwrap();
    assert!(matches!(block, ContentBlock::Unknown));
}

#[test]
fn input_message_helpers_serialize_as_string_content() {
    let msg = InputMessage::user("hi");
    let v = serde_json::to_value(&msg).unwrap();
    assert_eq!(v, json!({"role": "user", "content": "hi"}));
    roundtrip(&msg);
}

#[test]
fn input_message_with_block_content() {
    let msg = InputMessage {
        role: Role::User,
        content: MessageContent::Blocks(vec![ContentBlock::text("hello blocks")]),
    };
    let v = serde_json::to_value(&msg).unwrap();
    assert_eq!(
        v,
        json!({"role":"user","content":[{"type":"text","text":"hello blocks"}]})
    );
    roundtrip(&msg);
}

#[test]
fn message_create_params_minimal_shape() {
    let params = MessageCreateParams::builder()
        .model(model::CLAUDE_HAIKU_4_5)
        .max_tokens(64)
        .messages(vec![InputMessage::user("hi")])
        .build();

    let v: Value = serde_json::to_value(&params).unwrap();
    assert_eq!(v["model"], json!("claude-haiku-4-5"));
    assert_eq!(v["max_tokens"], json!(64));
    assert_eq!(v["messages"], json!([{"role":"user","content":"hi"}]));
    // Optional fields must not appear when unset.
    for absent in [
        "system",
        "temperature",
        "top_p",
        "top_k",
        "stop_sequences",
        "metadata",
    ] {
        assert!(v.get(absent).is_none(), "field {absent} should be omitted");
    }
}

#[test]
fn message_create_params_with_all_optionals() {
    let params = MessageCreateParams::builder()
        .model(model::CLAUDE_SONNET_4_6)
        .max_tokens(512)
        .messages(vec![InputMessage::user("hi")])
        .system(SystemPrompt::Text("be concise".into()))
        // 0.5 and 0.25 are exactly representable in IEEE-754, so the f32
        // value round-trips through serde_json's f64 numbers without loss.
        .temperature(0.5)
        .top_p(0.25)
        .top_k(50)
        .stop_sequences(vec!["\n\n".into()])
        .metadata(Metadata {
            user_id: Some("u_42".into()),
        })
        .build();

    let v: Value = serde_json::to_value(&params).unwrap();
    assert_eq!(v["system"], json!("be concise"));
    assert_eq!(v["temperature"], json!(0.5));
    assert_eq!(v["top_p"], json!(0.25));
    assert_eq!(v["top_k"], json!(50));
    assert_eq!(v["stop_sequences"], json!(["\n\n"]));
    assert_eq!(v["metadata"], json!({"user_id": "u_42"}));
}

#[test]
fn message_response_decodes_and_text_helper() {
    let body = json!({
        "id": "msg_01ABC",
        "type": "message",
        "role": "assistant",
        "content": [
            {"type": "text", "text": "Hello, "},
            {"type": "text", "text": "Edinaldo!"}
        ],
        "model": "claude-haiku-4-5",
        "stop_reason": "end_turn",
        "stop_sequence": null,
        "usage": {"input_tokens": 11, "output_tokens": 7}
    });

    let m: Message = serde_json::from_value(body.clone()).unwrap();
    assert_eq!(m.id, "msg_01ABC");
    assert_eq!(m.kind, "message");
    assert_eq!(m.role, Role::Assistant);
    assert_eq!(m.content.len(), 2);
    assert_eq!(m.stop_reason, Some(StopReason::EndTurn));
    assert_eq!(m.text(), "Hello, Edinaldo!");
    roundtrip(&m);
}

#[test]
fn api_error_body_decodes() {
    let body = json!({
        "type": "error",
        "error": {"type": "rate_limit_error", "message": "slow down"}
    });
    let parsed: ApiErrorBody = serde_json::from_value(body.clone()).unwrap();
    assert_eq!(parsed.kind, "error");
    assert_eq!(parsed.error.kind, "rate_limit_error");
    assert_eq!(parsed.error.message, "slow down");
}
