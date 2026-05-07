//! Model identifiers.
//!
//! [`Model`] is a transparent newtype around a string, so callers can pass any
//! ID the API accepts — including future models the SDK doesn't know about
//! yet — via [`Model::custom`]. Pre-baked constants for the current Claude
//! family are exposed as `pub const` items below.

// Each constant is named after its API ID; an extra `///` doc would just
// repeat that. Suppress the lint at module scope.
#![allow(missing_docs)]

use std::borrow::Cow;
use std::fmt;

use serde::{Deserialize, Serialize};

/// A Claude model identifier (e.g. `claude-opus-4-7`).
///
/// Use one of the `pub const` constants on this module for known models, or
/// [`Model::custom`] for anything not pre-baked.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Model(pub Cow<'static, str>);

impl Model {
    /// Wrap an arbitrary string as a model identifier.
    pub fn custom(id: impl Into<String>) -> Self {
        Self(Cow::Owned(id.into()))
    }

    /// The model identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&'static str> for Model {
    fn from(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

impl From<String> for Model {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

/// Mirrors the Go SDK's `ModelClaude*` constants in `message.go`.
/// Keep this list in sync with upstream when porting new model IDs.
pub const CLAUDE_OPUS_4_7: Model = Model(Cow::Borrowed("claude-opus-4-7"));
pub const CLAUDE_OPUS_4_6: Model = Model(Cow::Borrowed("claude-opus-4-6"));
pub const CLAUDE_OPUS_4_5: Model = Model(Cow::Borrowed("claude-opus-4-5"));
pub const CLAUDE_OPUS_4_5_20251101: Model = Model(Cow::Borrowed("claude-opus-4-5-20251101"));
pub const CLAUDE_OPUS_4_1: Model = Model(Cow::Borrowed("claude-opus-4-1"));
pub const CLAUDE_OPUS_4_1_20250805: Model = Model(Cow::Borrowed("claude-opus-4-1-20250805"));
pub const CLAUDE_OPUS_4_0: Model = Model(Cow::Borrowed("claude-opus-4-0"));
pub const CLAUDE_OPUS_4_20250514: Model = Model(Cow::Borrowed("claude-opus-4-20250514"));

pub const CLAUDE_SONNET_4_6: Model = Model(Cow::Borrowed("claude-sonnet-4-6"));
pub const CLAUDE_SONNET_4_5: Model = Model(Cow::Borrowed("claude-sonnet-4-5"));
pub const CLAUDE_SONNET_4_5_20250929: Model = Model(Cow::Borrowed("claude-sonnet-4-5-20250929"));
pub const CLAUDE_SONNET_4_0: Model = Model(Cow::Borrowed("claude-sonnet-4-0"));
pub const CLAUDE_SONNET_4_20250514: Model = Model(Cow::Borrowed("claude-sonnet-4-20250514"));

pub const CLAUDE_HAIKU_4_5: Model = Model(Cow::Borrowed("claude-haiku-4-5"));
pub const CLAUDE_HAIKU_4_5_20251001: Model = Model(Cow::Borrowed("claude-haiku-4-5-20251001"));

pub const CLAUDE_3_HAIKU_20240307: Model = Model(Cow::Borrowed("claude-3-haiku-20240307"));
