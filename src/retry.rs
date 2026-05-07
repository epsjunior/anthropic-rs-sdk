//! Retry policy.
//!
//! Skeleton only — v0 issues exactly one HTTP request per call and surfaces
//! transport/API failures verbatim. The shape lives here so that v0.2 retry
//! support (exponential backoff + jitter, `Retry-After` honoring, idempotency
//! check) is a localized change rather than a refactor of every call site.
//!
//! See `ROADMAP.md` for the full retry feature list.

use std::time::Duration;

/// How retries should be attempted.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct RetryPolicy {
    /// Total number of attempts including the first. `1` disables retries.
    pub max_attempts: u32,
    /// Base delay before the first retry; v0.2 will scale exponentially from this.
    pub initial_backoff: Duration,
    /// Upper bound on any single backoff.
    pub max_backoff: Duration,
}

impl Default for RetryPolicy {
    /// `max_attempts: 1` — no retries in v0.
    fn default() -> Self {
        Self {
            max_attempts: 1,
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(8),
        }
    }
}
