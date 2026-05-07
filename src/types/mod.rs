//! Public data types: request bodies, response bodies, content blocks, models.
//!
//! Organized by domain. The Messages API types live in [`messages`]; model
//! identifiers live in [`model`]. Tool-use and streaming types are reserved
//! modules for upcoming milestones (see `ROADMAP.md`).

pub mod messages;
pub mod model;
pub mod stream;
pub mod tools;

pub use messages::{
    ApiErrorBody, ContentBlock, ImageSource, InputMessage, Message, MessageContent,
    MessageCreateParams, Metadata, Role, StopReason, SystemPrompt, ToolResultContent, Usage,
};
pub use model::Model;
