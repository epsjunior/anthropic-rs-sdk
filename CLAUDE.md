# CLAUDE.md

Unofficial Anthropic SDK for Rust. Async-first HTTP client for the Anthropic Messages API, with planned support for SSE streaming and AWS Bedrock behind feature flags.

- **Crate name on crates.io:** `anthropic-rs-sdk`
- **Library name:** `anthropic` — users write `use anthropic::Client`
- **Edition:** Rust 2024
- **MSRV:** Rust 1.95
- **Reference SDK:** the official Go SDK in `../anthropic-sdk-go/` — port from there, not from scratch.

## Status (current shipped surface)

`v0.1` ships only `Messages.create` (non-streaming) plus model constants, env-loaded auth, and a typed `Error`. Streaming, tool use, multimodal (image/PDF), files, message batches, the beta surface (agents/sessions/memory/skills/vault), Bedrock and Vertex providers are **declared in the roadmap but not implemented yet** — see [`ROADMAP.md`](ROADMAP.md) for the full feature checklist with milestone tags.

This file is the source of truth for build commands, architecture, and conventions. Keep it short — promote deep guidance into `.claude/skills/` once a section outgrows ~30 lines or a sub-area (auth, streaming, tools) becomes a deep specialty.

## Commands

```bash
# Build (default features only)
cargo build

# Build with every feature (incl. Bedrock)
cargo build --all-features

# Test (no external services — uses the in-repo mock module)
cargo test --all-features

# Lint and format (must pass before any commit)
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings

# Docs build (warnings are errors)
cargo doc --no-deps --all-features -- -D warnings

# Supply-chain checks
cargo audit
cargo deny check

# Run an example against the in-repo mock server
cargo run --example mock_server -- --bind 127.0.0.1:8080
cargo run --example basic_messages
```

The canonical pre-commit gate is the `fmt` → `clippy` → `test` → `doc` → `audit` → `deny` chain above. **Always** run `cargo check` and `cargo clippy` before declaring work complete; never claim success without verifying compilation.

## Architecture

Single crate, organized as modules. We start flat and split into a workspace only if module ownership becomes painful.

### Module layout (`src/`)

- `client.rs` — `Client` and `ClientBuilder`. The single entry point. Holds the `reqwest::Client`, base URL, credentials.
- `error.rs` — `Error` enum (`thiserror`, `#[non_exhaustive]`), `Result<T> = std::result::Result<T, Error>`. Variants: `Http`, `Api { status, error_type, message, request_id, raw }`, `Serde`, `Config`. A `Stream` variant lands when streaming does (v0.2).
- `retry.rs` — `RetryPolicy` skeleton (no impl in v0; v0.2 wires retries here so no ad-hoc loops appear in callers).
- `auth.rs` — API-key handling, `secrecy::SecretString` wrapping, header construction; manual `Debug` impl that prints `<redacted>`.
- `http.rs` — request building, response parsing, common header logic, `request-id` extraction. Shared with the streaming path when it lands.
- `stream.rs` — **v0.2** — SSE decoding on top of `eventsource-stream`. Surfaces typed `MessageStreamEvent` deltas. Stub module today.
- `messages.rs` — `Client::messages()` API surface. Non-streaming `create()` shipped; streaming variant in v0.2.
- `types/` — pure data types split by domain:
  - `types/messages.rs` — `MessageCreateParams`, `Message`, `ContentBlock`, `Role`, `StopReason`, `Usage`, `InputMessage`, `MessageContent`, `SystemPrompt`, `ImageSource`, `ToolResultContent`, `Metadata`, `ApiErrorBody`.
  - `types/model.rs` — `Model` newtype (`Cow<'static, str>`) + named constants (`CLAUDE_OPUS_4_7`, `CLAUDE_SONNET_4_6`, `CLAUDE_SONNET_4_5`, `CLAUDE_HAIKU_4_5`, plus dated variants and the older 4.x line). `Model::custom("…")` escape hatch for unknown IDs.
  - `types/tools.rs` — **v0.2** — tool definitions, tool-use / tool-result helpers. Stub today.
  - `types/stream.rs` — **v0.2** — stream event variants. Stub today.
- `bedrock/` — **v0.4** — planned, gated by `feature = "bedrock"`. Will host the Bedrock provider adapter: SigV4 signing, region/model-id mapping, the same `messages` API rebound onto Bedrock's transport. Not yet present in `src/`.

### Surface area

- The `Client` builder is the single entry point. All requests go through it; no free functions that issue HTTP.
- Streaming and non-streaming share one `MessagesRequest`, distinguished by `.stream()` returning a `MessagesStream` (impl `Stream<Item = Result<MessageStreamEvent>>`) versus `.send().await` returning `MessagesResponse`.
- All retry/backoff goes through `retry::RetryPolicy`. No ad-hoc retry loops in callers.
- Public surface is `pub` only at module root re-exports in `lib.rs`. Internal helpers stay `pub(crate)`.

The Go SDK in the sibling `../anthropic-sdk-go/` directory is the reference for API surface naming and feature parity. Mirror its naming where Rust idioms allow.

## Conventions

### Error handling
- Library code: `thiserror::Error` on typed enums with `#[from]`. Mark public error enums `#[non_exhaustive]` so adding a variant is not a breaking change.
- Binaries (examples, mock server): `anyhow::Result<T>` with `.context(...)` at boundaries.
- **Never** `.unwrap()` or `.expect()` in library code. `.expect()` is permitted only at process-startup invariants in binaries with a descriptive message.
- Map foreign errors with context: `.map_err(|e| Error::Transport { source: e })?`.

### Async
- Tokio is the runtime. Spawned tasks must be awaited or aborted — no fire-and-forget.
- **Never** hold a `std::sync::Mutex` guard across `.await`. Use `tokio::sync::Mutex` when a guard must cross suspension, otherwise scope the lock tightly.
- **Never** `block_on` inside async contexts. Offload CPU-bound work to `tokio::task::spawn_blocking`.
- Long-running loops honor cancellation via `tokio::select!` against a shutdown signal.

### Public API shape
- Prefer owned types (`String`, `Vec<T>`, `Bytes`) on public function signatures; borrow inside implementations.
- Builder pattern for anything with more than ~3 fields. Builders return `Result` on `.build()` only when validation can fail.
- Builders use the [`bon`](https://docs.rs/bon) crate so `serde` annotations and builder annotations live on a single struct (no shadow types).
- Every `pub` item gets a `///` doc comment. Field- and variant-level docs are added selectively when names aren't self-explanatory; verbose modules use `#![allow(missing_docs)]` and rely on the type-level documentation.

### Unsafe
- `#![deny(unsafe_code)]` at the crate root. Any `unsafe` block requires a `// SAFETY:` comment explaining invariant preservation, and is reviewed.

### Secrets
- Use `secrecy::Secret<String>` for API keys. Call `.expose_secret()` only at the HTTP-header construction site.
- Implement `Debug` manually as `[REDACTED]` for any type that holds credentials.
- API keys come from `ANTHROPIC_API_KEY` (or per-provider equivalents) via `dotenvy` for examples and `std::env` in production. **Never** commit `.env`. `.env` is in `.gitignore`.
- **Never** log request bodies, headers, or response bodies that may contain user content or keys. Use `tracing` spans with redacted fields.

### Dependencies
- Use `cargo add <dep>` rather than editing `Cargo.toml` by hand.
- Pin the major version (`reqwest = "0.12"`), not a single point release, unless there's a known regression.
- Run `cargo machete` periodically; remove unused deps in the same PR that orphans them.
- When the project splits into a workspace, move all version pins to root `[workspace.dependencies]` and reference them from members with `{ workspace = true }`. Until then, deps live in the single `Cargo.toml`.

### Feature flags
- `default = ["rustls-tls"]`. TLS backend is the only on-by-default opinion. `rustls-tls` and `native-tls` are mutually exclusive.
- `bedrock` is **off** by default; turning it on pulls in `aws-config`, `aws-credential-types`, and `aws-sigv4`.
- Every meaningful feature combination must compile. CI runs `--no-default-features`, default, `--features bedrock`, and `--all-features`.

### Imports
- Use `crate::` for cross-module imports. Do **not** route through `super::super::`.
- When a module is later extracted into its own crate, switch every import to the extracted crate name in the same PR — don't leave `crate::foo::` aliasing the new crate.

### Logging
- `tracing` is the logging facade. `tracing::error!` for failures, `debug!` for internal diagnostics, `info!` only for events a downstream operator would care about. **Never** `println!` outside of examples.

## Testing

- Unit tests live next to the code they test (`#[cfg(test)] mod tests { ... }`).
- Integration tests live in `tests/` and exercise the public API only.
- **All** integration tests run against an in-process mock (`wiremock` or a `tower::Service` fake) without network access. CI runs with no API key set.
- A `live-tests` feature flag gates tests that hit the real Anthropic API. These run only on a manual dispatch workflow with a scoped key.
- Mocks must capture every argument production callers pass. Asserting "a call happened" is insufficient — assert on the request body, headers, and any retry count that matters.
- Flaky tests get fixed, never `#[ignore]`'d.

## Planning discipline

When asked to design a non-trivial change, present a plan for review **before** implementing. The first response on a new feature should be a written plan, not a diff. Don't declare a task done until `cargo test --all-features` passes and the verification gate above is clean.

## Pointers

- **[`ROADMAP.md`](ROADMAP.md)** — single source of truth for shipped vs. planned features, with milestone tags. Tick boxes here when work lands.
- Architecture deep-dives: `docs/dev/architecture/` (create as the SDK grows).
- Reusable agent skills: `.claude/skills/` (promote sections of this file here once they exceed ~30 lines or specialize per-area).
- Reference SDK for API surface parity: `../anthropic-sdk-go/` (official Anthropic Go SDK in the sibling directory; mirror naming where idiomatic).

## Git

- Commits authored as `Edinaldo Junior <epsjunior@icloud.com>` — no `Co-Authored-By` line.
- Conventional Commits format: `feat(client): ...`, `fix(stream): ...`, `chore: ...`.
- Don't commit `Cargo.lock` deletions or formatting-only churn alongside feature work.
