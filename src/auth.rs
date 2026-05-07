//! Credential handling.
//!
//! Wraps the API key in [`secrecy::SecretString`] so it cannot be accidentally
//! logged via `Debug`/`Display`, and centralizes header construction so the
//! key is exposed only at the point it is written into HTTP headers.

use std::fmt;

use http::HeaderMap;
use http::header::{HeaderName, HeaderValue};
use secrecy::{ExposeSecret, SecretString};

/// API credentials for the Anthropic REST API.
#[derive(Clone)]
pub struct Credentials {
    api_key: SecretString,
}

impl Credentials {
    /// Wrap a raw API key.
    pub fn from_api_key(key: impl Into<String>) -> Self {
        Self {
            api_key: SecretString::from(key.into()),
        }
    }

    /// Inject `x-api-key` into the supplied header map. The plaintext key
    /// crosses module boundaries only inside this function.
    pub(crate) fn write_into(&self, headers: &mut HeaderMap) -> crate::Result<()> {
        let value = HeaderValue::from_str(self.api_key.expose_secret()).map_err(|_| {
            crate::Error::Config("API key contains characters invalid for an HTTP header".into())
        })?;
        headers.insert(HeaderName::from_static("x-api-key"), value);
        Ok(())
    }
}

/// Custom `Debug` so credentials never leak into logs / panic messages.
impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("api_key", &"<redacted>")
            .finish()
    }
}
