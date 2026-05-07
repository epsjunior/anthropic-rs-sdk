//! # anthropic — community Rust SDK for the Anthropic API
//!
//! This crate is an early port of the official
//! [`anthropic-sdk-go`](https://github.com/anthropics/anthropic-sdk-go) to Rust.
//! v0 implements only the non-streaming Messages API; streaming, tool use,
//! multimodal, files, batches, Bedrock, Vertex, and the beta surface are
//! tracked in `ROADMAP.md`.
//!
//! ## Quickstart
//!
//! ```no_run
//! use anthropic::types::{InputMessage, MessageCreateParams, model};
//!
//! # async fn run() -> anthropic::Result<()> {
//! // Reads ANTHROPIC_API_KEY from the environment.
//! let client = anthropic::Client::from_env()?;
//!
//! let response = client
//!     .messages()
//!     .create(
//!         MessageCreateParams::builder()
//!             .model(model::CLAUDE_HAIKU_4_5)
//!             .max_tokens(256)
//!             .messages(vec![InputMessage::user("Say hi in one short sentence.")])
//!             .build(),
//!     )
//!     .await?;
//!
//! println!("{}", response.text());
//! # Ok(()) }
//! ```
//!
//! ## Multi-turn
//!
//! ```no_run
//! use anthropic::types::{InputMessage, MessageCreateParams, model};
//!
//! # async fn run() -> anthropic::Result<()> {
//! let client = anthropic::Client::from_env()?;
//! let messages = vec![
//!     InputMessage::user("My name is Edinaldo."),
//!     InputMessage::assistant("Hello, Edinaldo! How can I help?"),
//!     InputMessage::user("What name did I just give you?"),
//! ];
//! let response = client
//!     .messages()
//!     .create(
//!         MessageCreateParams::builder()
//!             .model(model::CLAUDE_SONNET_4_6)
//!             .max_tokens(256)
//!             .messages(messages)
//!             .build(),
//!     )
//!     .await?;
//! println!("{}", response.text());
//! # Ok(()) }
//! ```
//!
//! ## Errors
//!
//! All fallible operations return [`Result<T>`] (alias of `Result<T, Error>`).
//! Non-2xx responses become [`Error::Api`] with the HTTP status, the parsed
//! [`ErrorType`], the server-supplied message, and the `request-id` header.

#![deny(unsafe_code)]
#![warn(missing_docs)]
// Field- and variant-level docs are added selectively where the name isn't
// self-documenting; we don't blanket-warn on every `pub` field for v0.
#![allow(clippy::missing_docs_in_private_items)]

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod http;
pub mod messages;
pub mod retry;
pub mod stream;
pub mod types;

pub use crate::client::{Client, ClientBuilder};
pub use crate::error::{Error, ErrorType, Result};
pub use crate::messages::MessagesService;
pub use crate::types::messages::{
    ApiErrorBody, ContentBlock, ImageSource, InputMessage, Message, MessageContent,
    MessageCreateParams, Metadata, Role, StopReason, SystemPrompt, ToolResultContent, Usage,
};
pub use crate::types::model::{self, Model};
