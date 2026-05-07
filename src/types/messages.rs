//! Messages API request and response types.
//!
//! Mirrors the Go SDK's `message.go` for the non-streaming surface. Streaming
//! event types live in [`crate::types::stream`] (v0.2). Tool-use blocks have
//! placeholder support here but the dedicated tool-runner lives in
//! [`crate::types::tools`] (v0.2).

// Field-level docs are added selectively; the type-level documentation
// covers the common case.
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::model::Model;

// =====================================================================
// Roles & stop reasons
// =====================================================================

/// Conversation role on a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
}

/// Why the model stopped generating.
///
/// New variants may be added by the API; the [`StopReason::Other`] catch-all
/// keeps deserialization forward-compatible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    StopSequence,
    ToolUse,
    PauseTurn,
    Refusal,
    #[serde(other)]
    Other,
}

// =====================================================================
// Usage
// =====================================================================

/// Token accounting for a single response.
///
/// Cache fields are present only when prompt caching (v0.2 feature) is in use.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_creation_input_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_read_input_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
}

// =====================================================================
// Content blocks
// =====================================================================

/// One content block in a message.
///
/// Internally tagged on the `type` field. Unknown server-side block types
/// deserialize as [`ContentBlock::Unknown`] so the SDK doesn't break when the
/// API gains a new block type before we've shipped support for it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text {
        text: String,
    },
    Image {
        source: ImageSource,
    },
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    ToolResult {
        tool_use_id: String,
        content: ToolResultContent,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
    },
    Thinking {
        thinking: String,
        signature: String,
    },
    RedactedThinking {
        data: String,
    },
    /// Forward-compat catch-all for block types added by the API after this
    /// SDK was published. The original payload is dropped.
    #[serde(other)]
    Unknown,
}

impl ContentBlock {
    /// Convenience constructor for the most common case.
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }
}

/// Source for an [`ContentBlock::Image`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ImageSource {
    Base64 { media_type: String, data: String },
    Url { url: String },
    File { file_id: String },
}

/// Body of a `tool_result` block. Either plain text or a sub-list of blocks
/// (e.g. text + image return).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolResultContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

// =====================================================================
// Messages (request side)
// =====================================================================

/// One conversation turn sent to the API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputMessage {
    pub role: Role,
    pub content: MessageContent,
}

impl InputMessage {
    /// Build a user message from a plain string.
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::Text(text.into()),
        }
    }

    /// Build an assistant message from a plain string.
    pub fn assistant(text: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: MessageContent::Text(text.into()),
        }
    }
}

/// Either a sugar string or an explicit block array — the API accepts both.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

impl From<String> for MessageContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for MessageContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_owned())
    }
}

impl From<Vec<ContentBlock>> for MessageContent {
    fn from(b: Vec<ContentBlock>) -> Self {
        Self::Blocks(b)
    }
}

/// Either a sugar string or an explicit block array for the system prompt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemPrompt {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

impl From<String> for SystemPrompt {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for SystemPrompt {
    fn from(s: &str) -> Self {
        Self::Text(s.to_owned())
    }
}

/// Optional metadata attached to a request — surfaces in usage analytics.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Body for `POST /v1/messages` (non-streaming).
///
/// Use the builder ([`MessageCreateParams::builder`]) to construct one — `model`,
/// `max_tokens`, and `messages` are required; everything else is optional.
#[derive(Debug, Clone, Serialize, bon::Builder)]
pub struct MessageCreateParams {
    pub model: Model,
    pub max_tokens: u32,
    pub messages: Vec<InputMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemPrompt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

// =====================================================================
// Messages (response side)
// =====================================================================

/// A response from `POST /v1/messages`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    /// Always `"message"` for this endpoint; preserved for round-tripping.
    #[serde(rename = "type")]
    pub kind: String,
    pub role: Role,
    pub content: Vec<ContentBlock>,
    pub model: String,
    #[serde(default)]
    pub stop_reason: Option<StopReason>,
    #[serde(default)]
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

impl Message {
    /// Concatenate all `text` blocks into a single string. Useful for the
    /// common case of "I just want the model's reply".
    ///
    /// Non-text blocks (tool_use, image, etc.) are skipped.
    pub fn text(&self) -> String {
        let mut out = String::new();
        for block in &self.content {
            if let ContentBlock::Text { text } = block {
                out.push_str(text);
            }
        }
        out
    }
}

// =====================================================================
// Error body (parsed from non-2xx responses)
// =====================================================================

/// Shape of the JSON error body returned by the Anthropic API.
///
/// Wrapped inside [`crate::Error::Api`] when an HTTP response is non-2xx.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiErrorBody {
    /// Always `"error"`.
    #[serde(rename = "type")]
    pub kind: String,
    pub error: ApiErrorPayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiErrorPayload {
    #[serde(rename = "type")]
    pub kind: String,
    pub message: String,
}
