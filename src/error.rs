//! Error types.
//!
//! All public error reporting flows through [`Error`]. The variant set is
//! marked `#[non_exhaustive]` so adding a new failure mode (e.g. `Stream`
//! when streaming lands in v0.2) is not a breaking change.

use serde::{Deserialize, Serialize};

use crate::types::messages::ApiErrorBody;

/// Crate-wide result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Every failure mode the SDK surfaces.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Transport-level failure from `reqwest` (DNS, TLS handshake, connection
    /// reset, response framing). Body decoding errors that come through
    /// `reqwest::Response::json` also land here.
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// The server returned a non-2xx status. The `request_id` header is
    /// preserved so callers can quote it in support tickets.
    #[error("API error {status}{}: {message}", error_type.as_ref().map(|t| format!(" ({t:?})")).unwrap_or_default())]
    Api {
        /// HTTP status returned by the server.
        status: http::StatusCode,
        /// Parsed `error.type` from the response body, if it matched a known
        /// category; `None` when the body was non-JSON.
        error_type: Option<ErrorType>,
        /// Human-readable message lifted from `error.message`.
        message: String,
        /// Value of the `request-id` response header, if present.
        request_id: Option<String>,
        /// Raw response body — present even when JSON decoding succeeded so
        /// callers can log the original payload.
        raw: String,
    },

    /// JSON serialization failed (request body) or deserialization failed
    /// (response body) with a 2xx status. 4xx/5xx bodies fall under
    /// [`Error::Api`] regardless of whether the body parsed.
    #[error("JSON encode/decode error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Bad client configuration (e.g. missing API key, malformed base URL).
    /// Surfaced from `Client::from_env`, `ClientBuilder::build`, etc.
    #[error("invalid configuration: {0}")]
    Config(String),
}

/// Categorized error types returned in the `error.type` field of the
/// API's JSON error body. Mirrors the Go SDK's `shared/constant` enum.
///
/// New API error categories deserialize as [`ErrorType::Unknown`] so the
/// SDK doesn't break when the API gains a new error variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[allow(missing_docs)] // variant names mirror the upstream API strings 1:1
pub enum ErrorType {
    InvalidRequestError,
    AuthenticationError,
    BillingError,
    PermissionError,
    NotFoundError,
    RateLimitError,
    TimeoutError,
    ApiError,
    OverloadedError,
    GatewayTimeoutError,
    /// Catch-all for new categories the API may add.
    #[serde(other)]
    Unknown,
}

/// Build an [`Error::Api`] from a parsed body, falling back to the raw text
/// when the JSON shape didn't match.
pub(crate) fn api_error_from_response(
    status: http::StatusCode,
    request_id: Option<String>,
    raw: String,
) -> Error {
    if let Ok(body) = serde_json::from_str::<ApiErrorBody>(&raw) {
        let error_type =
            serde_json::from_value::<ErrorType>(serde_json::Value::String(body.error.kind.clone()))
                .ok();
        Error::Api {
            status,
            error_type,
            message: body.error.message,
            request_id,
            raw,
        }
    } else {
        Error::Api {
            status,
            error_type: None,
            message: format!("non-JSON error body (HTTP {})", status.as_u16()),
            request_id,
            raw,
        }
    }
}
