# anthropic-rs-sdk

> **Status: v0.1 — non-streaming Messages API only.** This crate is an early
> port of the official [Anthropic Go SDK][go-sdk] to Rust. See
> [`ROADMAP.md`](ROADMAP.md) for the full feature checklist.

Async Rust SDK for the Anthropic API. Built on `reqwest` + `tokio`, with a
typed request/response model, forward-compatible enums, redacted credentials,
and `wiremock`-driven tests so CI never touches the real API.

```toml
[dependencies]
anthropic-rs-sdk = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The library name is `anthropic`, so `use anthropic::Client` works regardless
of the crate name on crates.io.

## Quickstart

```rust,no_run
use anthropic::types::{InputMessage, MessageCreateParams, model};

#[tokio::main]
async fn main() -> anthropic::Result<()> {
    // Reads ANTHROPIC_API_KEY (and optional ANTHROPIC_BASE_URL).
    let client = anthropic::Client::from_env()?;

    let response = client
        .messages()
        .create(
            MessageCreateParams::builder()
                .model(model::CLAUDE_HAIKU_4_5)
                .max_tokens(256)
                .messages(vec![InputMessage::user("Hello, Claude!")])
                .build(),
        )
        .await?;

    println!("{}", response.text());
    Ok(())
}
```

Run the bundled examples:

```bash
ANTHROPIC_API_KEY=sk-ant-... cargo run --example basic_message
ANTHROPIC_API_KEY=sk-ant-... cargo run --example multi_turn
```

## What's in v0.1

- `Client::from_env()` — reads `ANTHROPIC_API_KEY` (+ optional `ANTHROPIC_BASE_URL`)
- `Client::builder()` — explicit `api_key`, `base_url`, `timeout`, `http_client`
- `messages.create` — non-streaming, with multi-turn `Vec<InputMessage>`
- Model constants for the current Claude family (Opus 4.7, Sonnet 4.6, Haiku 4.5, plus dated variants and the older 4.x line)
- `Model::custom("...")` escape hatch for unknown IDs
- Typed `Error` with `Error::Api { status, error_type, message, request_id, raw }`
- `request-id` preserved on every error response
- `secrecy::SecretString`-wrapped API key with redacted `Debug`
- 21 tests passing — 14 serde round-trips, 4 wiremock end-to-end, 3 doctests
- Live-API integration test gated by `--features live-tests`

## What's *not* in v0.1

Streaming, tool use, multimodal (image / PDF), files, message batches, the
beta surface (agents / sessions / memory / skills / vault / user profiles),
Bedrock + Vertex providers, retries with backoff. Each of these has a tracked
checkbox in [`ROADMAP.md`](ROADMAP.md) — the goal is full
[Go SDK][go-sdk] parity by `v1.0`.

## Cargo features

| Feature      | Default | Effect                                                     |
|--------------|---------|------------------------------------------------------------|
| `rustls-tls` |   yes   | TLS via `rustls`. Pure Rust, easy cross-compile.           |
| `native-tls` |   no    | TLS via the OS trust store. Mutually exclusive with above. |
| `streaming`  |   no    | (Reserved) SSE streaming support. Lands in v0.2.           |
| `tools`      |   no    | (Reserved) Tool-use helpers. Lands in v0.2.                |
| `bedrock`    |   no    | (Reserved) AWS Bedrock provider. Lands in v0.4.            |
| `vertex`     |   no    | (Reserved) Google Vertex AI provider. Lands in v0.4.       |
| `live-tests` |   no    | Enables `tests/messages_live.rs` against the real API.     |

## Verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test                                # no network, no API key
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
ANTHROPIC_API_KEY=sk-ant-... cargo test --features live-tests
```

## Project conventions

Contributor-facing conventions live in [`CLAUDE.md`](CLAUDE.md): error
handling, async rules, secrets handling, testing strategy, dependency hygiene.

## License

MIT — see [`LICENSE`](LICENSE).

## Why "anthropic-rs-sdk"?

`anthropic`, `anthropic-sdk`, and `anthropic-rs` were all taken on crates.io
by stale community crates last updated in 2024 (none of which support the
Claude 4 family). `anthropic-rs-sdk` keeps the convention close to
[`anthropic-sdk-go`][go-sdk] while making it obvious this is the Rust port.

[go-sdk]: https://github.com/anthropics/anthropic-sdk-go
