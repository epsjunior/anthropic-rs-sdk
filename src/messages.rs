//! The Messages API surface.
//!
//! v0 implements only [`MessagesService::create`] (non-streaming). Streaming,
//! token counting, and message batches are tracked in `ROADMAP.md`.

use crate::client::Client;
use crate::error::{Error, Result};
use crate::http::{default_headers, post_json};
use crate::types::messages::{Message, MessageCreateParams};

/// Resource handle for `/v1/messages` endpoints. Obtained via [`Client::messages`].
#[derive(Debug)]
pub struct MessagesService<'c> {
    client: &'c Client,
}

impl<'c> MessagesService<'c> {
    pub(crate) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// `POST /v1/messages` — create a single message (non-streaming).
    ///
    /// # Example
    /// ```no_run
    /// # async fn run() -> anthropic::Result<()> {
    /// use anthropic::types::{InputMessage, MessageCreateParams, model};
    ///
    /// let client = anthropic::Client::from_env()?;
    /// let response = client
    ///     .messages()
    ///     .create(
    ///         MessageCreateParams::builder()
    ///             .model(model::CLAUDE_HAIKU_4_5)
    ///             .max_tokens(256)
    ///             .messages(vec![InputMessage::user("Hello, Claude!")])
    ///             .build(),
    ///     )
    ///     .await?;
    /// println!("{}", response.text());
    /// # Ok(()) }
    /// ```
    pub async fn create(&self, params: MessageCreateParams) -> Result<Message> {
        let url = self
            .client
            .inner
            .base_url
            .join("v1/messages")
            .map_err(|e| Error::Config(format!("failed to build /v1/messages URL: {e}")))?;
        let headers = default_headers(&self.client.inner.credentials)?;
        post_json(&self.client.inner.http, url, headers, &params).await
    }
}
