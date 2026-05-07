//! Static configuration values and environment-variable resolution.

/// Default base URL when `ANTHROPIC_BASE_URL` is unset.
pub const DEFAULT_BASE_URL: &str = "https://api.anthropic.com";

/// Value sent in the `anthropic-version` header on every request.
///
/// Mirrors the Go SDK's stable production version. Bump only when intentionally
/// adopting a newer API revision (and update the changelog).
pub const ANTHROPIC_VERSION: &str = "2023-06-01";

/// `User-Agent` header value. Generated at compile time so it stays in sync
/// with `Cargo.toml`.
pub const USER_AGENT: &str = concat!("anthropic-rs-sdk/", env!("CARGO_PKG_VERSION"));

/// Environment variable for the API key.
pub const ENV_API_KEY: &str = "ANTHROPIC_API_KEY";

/// Environment variable for an alternate base URL.
pub const ENV_BASE_URL: &str = "ANTHROPIC_BASE_URL";
