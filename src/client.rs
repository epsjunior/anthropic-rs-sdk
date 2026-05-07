//! HTTP client for the Anthropic API.
//!
//! Construct a [`Client`] with [`Client::from_env`] (reads `ANTHROPIC_API_KEY`
//! and optionally `ANTHROPIC_BASE_URL`) or [`Client::builder`] for explicit
//! configuration. The client is cheap to clone — internally it shares a
//! reference-counted `reqwest::Client` and credential set.

use std::sync::Arc;
use std::time::Duration;

use url::Url;

use crate::auth::Credentials;
use crate::config::{DEFAULT_BASE_URL, ENV_API_KEY, ENV_BASE_URL};
use crate::error::{Error, Result};
use crate::messages::MessagesService;

/// Top-level handle for the Anthropic API.
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) inner: Arc<ClientInner>,
}

#[derive(Debug)]
pub(crate) struct ClientInner {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: Url,
    pub(crate) credentials: Credentials,
}

impl Client {
    /// Build a [`Client`] from environment variables.
    ///
    /// Required: `ANTHROPIC_API_KEY`.
    /// Optional: `ANTHROPIC_BASE_URL` (defaults to `https://api.anthropic.com`).
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var(ENV_API_KEY)
            .map_err(|_| Error::Config(format!("environment variable {ENV_API_KEY} is not set")))?;
        let mut builder = Self::builder().api_key(api_key);
        if let Ok(url) = std::env::var(ENV_BASE_URL) {
            builder = builder.base_url(
                Url::parse(&url)
                    .map_err(|e| Error::Config(format!("invalid {ENV_BASE_URL}: {e}")))?,
            );
        }
        builder.build()
    }

    /// Start a fluent builder for explicit construction.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Access the Messages API.
    pub fn messages(&self) -> MessagesService<'_> {
        MessagesService::new(self)
    }
}

/// Builder for [`Client`]. Use [`Client::builder`] to start one.
#[derive(Debug, Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<Url>,
    timeout: Option<Duration>,
    http_client: Option<reqwest::Client>,
}

impl ClientBuilder {
    /// Set the API key. Required.
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Override the API base URL. Defaults to `https://api.anthropic.com`.
    pub fn base_url(mut self, url: Url) -> Self {
        self.base_url = Some(url);
        self
    }

    /// Override the request timeout. Defaults to whatever `reqwest` uses
    /// when no explicit `http_client` is supplied (no timeout).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Provide a fully-configured `reqwest::Client` (for custom proxies,
    /// connection pools, root certs, etc.). When set, [`ClientBuilder::timeout`]
    /// is ignored.
    pub fn http_client(mut self, http: reqwest::Client) -> Self {
        self.http_client = Some(http);
        self
    }

    /// Finalize the builder.
    pub fn build(self) -> Result<Client> {
        let api_key = self
            .api_key
            .ok_or_else(|| Error::Config("api_key is required".into()))?;
        if api_key.trim().is_empty() {
            return Err(Error::Config("api_key must not be empty".into()));
        }

        let base_url = match self.base_url {
            Some(u) => u,
            None => Url::parse(DEFAULT_BASE_URL)
                .map_err(|e| Error::Config(format!("default base URL is invalid: {e}")))?,
        };

        let http = match self.http_client {
            Some(c) => c,
            None => {
                let mut b = reqwest::Client::builder();
                if let Some(t) = self.timeout {
                    b = b.timeout(t);
                }
                b.build().map_err(Error::Http)?
            }
        };

        Ok(Client {
            inner: Arc::new(ClientInner {
                http,
                base_url,
                credentials: Credentials::from_api_key(api_key),
            }),
        })
    }
}
