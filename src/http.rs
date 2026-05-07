//! Internal request/response plumbing.
//!
//! Builds a `reqwest::RequestBuilder` with the standard Anthropic headers,
//! sends it, and either parses the JSON body or maps a non-2xx status into
//! [`crate::Error::Api`]. The `request-id` response header is preserved on
//! every error so callers can quote it in support tickets.

use http::StatusCode;
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use crate::auth::Credentials;
use crate::config::{ANTHROPIC_VERSION, USER_AGENT};
use crate::error::{Error, Result, api_error_from_response};

/// Header name the API uses to surface a per-request trace ID.
const REQUEST_ID_HEADER: &str = "request-id";

/// Build the static set of headers attached to every request.
pub(crate) fn default_headers(creds: &Credentials) -> Result<http::HeaderMap> {
    let mut headers = http::HeaderMap::new();
    headers.insert(
        http::header::ACCEPT,
        http::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        http::HeaderName::from_static("anthropic-version"),
        http::HeaderValue::from_static(ANTHROPIC_VERSION),
    );
    headers.insert(
        http::header::USER_AGENT,
        http::HeaderValue::from_static(USER_AGENT),
    );
    creds.write_into(&mut headers)?;
    Ok(headers)
}

/// Issue a `POST` with a JSON body and decode the JSON response into `R`.
///
/// Non-2xx statuses become [`Error::Api`]; transport errors stay as
/// [`Error::Http`]; JSON failures on a 2xx body become [`Error::Serde`].
pub(crate) async fn post_json<B, R>(
    http: &reqwest::Client,
    url: Url,
    headers: http::HeaderMap,
    body: &B,
) -> Result<R>
where
    B: Serialize + ?Sized,
    R: DeserializeOwned,
{
    let body_bytes = serde_json::to_vec(body)?;

    let response = http
        .post(url)
        .headers(headers)
        .body(body_bytes)
        .send()
        .await?;

    let status = response.status();
    let request_id = extract_request_id(response.headers());
    let raw = response.text().await?;

    if status.is_success() {
        serde_json::from_str::<R>(&raw).map_err(Error::from)
    } else {
        Err(api_error_from_response(status, request_id, raw))
    }
}

/// Convert a `reqwest::StatusCode` (which is `http::StatusCode` in v0.12+)
/// without an extra dependency on `reqwest`'s re-export.
#[inline]
#[allow(dead_code)]
pub(crate) fn status_to_http(s: reqwest::StatusCode) -> StatusCode {
    StatusCode::from_u16(s.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

fn extract_request_id(headers: &http::HeaderMap) -> Option<String> {
    headers
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned())
}
