# Roadmap — anthropic-rs-sdk

This file is the **single source of truth** for what's shipped and what's
still missing relative to the official
[Anthropic Go SDK](https://github.com/anthropics/anthropic-sdk-go). Every Go
SDK package and helper has at least one entry below.

## Status legend

- `[x]` shipped
- `[~]` in progress
- `[ ]` planned
- `[-]` explicitly out of scope

Each line ends with a milestone tag in parentheses. The current version is
`v0.1.0`.

## Milestones

| Milestone | Theme |
|-----------|-------|
| **v0.1**  | Messages.create non-streaming, env auth, model constants, typed errors, examples, mock + live tests |
| **v0.2**  | Streaming, tool use, multimodal (images), token counting, prompt caching, retries with backoff |
| **v0.3**  | Files API, message batches, structured outputs, citations, server-side tools (web_search/computer/bash/code_execution) |
| **v0.4**  | Workspace split + Bedrock + Vertex providers + AWS external gateway |
| **v0.5**  | Beta surface: agents, sessions, memory stores, skills, vault, user profiles |
| **v1.0**  | Full Go-SDK parity, API stability pledge, semver promise |

---

## Messages API

- [x] `messages.create` non-streaming (`MessagesService::create`)  *(v0.1)*
- [x] Multi-turn conversation via `Vec<InputMessage>`  *(v0.1)*
- [x] `system` prompt — string or block array  *(v0.1)*
- [x] `temperature`, `top_p`, `top_k`, `stop_sequences`, `metadata`  *(v0.1)*
- [x] Forward-compatible `ContentBlock::Unknown`  *(v0.1)*
- [ ] `messages.create_streaming` (SSE)  *(v0.2)*
- [ ] `messages.count_tokens`  *(v0.2)*
- [ ] Image inputs — base64 / URL / `file_id`  *(v0.2)*
- [ ] PDF / document inputs  *(v0.2)*
- [ ] Tool use — `Tool`, `tool_choice`, `tool_use` / `tool_result` blocks  *(v0.2)*
- [ ] Server-side tools — `web_search`, `computer`, `bash`, `code_execution`  *(v0.3)*
- [ ] Extended thinking — regular + interleaved + redacted  *(v0.2)*
- [ ] Citations — `char_location`, `page_location`, `content_block_location`  *(v0.3)*
- [ ] Prompt caching — `cache_control` on system/messages/tools  *(v0.2)*
- [ ] Structured outputs — `response_format` JSON schema + auto-parse  *(v0.3)*

## Models API

- [x] Model constants (`model::CLAUDE_OPUS_4_7` etc.) and `Model::custom`  *(v0.1)*
- [ ] `models.list` — paginated  *(v0.2)*
- [ ] `models.get(id)`  *(v0.2)*

## Streaming infrastructure

- [ ] SSE parser on `eventsource-stream`  *(v0.2)*
- [ ] `MessageStreamEvent` enum (message_start/delta/stop, content_block_start/delta/stop, ping, error)  *(v0.2)*
- [ ] `Message::accumulate` rebuild helper  *(v0.2)*
- [ ] Backpressure-friendly `Stream` impl  *(v0.2)*
- [ ] Custom decoder registration  *(v0.4)*

## Message Batches

- [ ] `batches.create`  *(v0.3)*
- [ ] `batches.get(id)`  *(v0.3)*
- [ ] `batches.list` + auto-pager  *(v0.3)*
- [ ] `batches.delete(id)`  *(v0.3)*
- [ ] `batches.cancel(id)`  *(v0.3)*
- [ ] `batches.results(id)` — JSONL stream  *(v0.3)*

## Files API (Beta)

- [ ] `files.upload` — multipart  *(v0.3)*
- [ ] `files.list` + auto-pager  *(v0.3)*
- [ ] `files.get(id)`  *(v0.3)*
- [ ] `files.delete(id)`  *(v0.3)*
- [ ] `files.download(id)` — binary stream  *(v0.3)*
- [ ] `anthropic-beta: files-api-2025-04-14` header opt-in  *(v0.3)*

## Tool Runner (Beta)

- [ ] `Tool` definition with JSON Schema  *(v0.2)*
- [ ] Schema generation from Rust structs (via `schemars`)  *(v0.2)*
- [ ] `ToolRunner::run_to_completion` loop  *(v0.2)*
- [ ] Streaming variant of the tool runner  *(v0.2)*

## Beta — Agents (`Beta.Agents` and friends)

- [ ] `agents.create / get / list / update / delete / versions`  *(v0.5)*
- [ ] `environments.create / get / list / update / delete`  *(v0.5)*
- [ ] `sessions.create / get / list / delete` + `sessions.events.send / stream`  *(v0.5)*
- [ ] `vaults.create / get / list / delete` + `vaults.credentials.*`  *(v0.5)*
- [ ] `memory_stores.create / get / list / delete` + `memories.*` + `memories.versions.*`  *(v0.5)*
- [ ] `skills.create / get / list / delete / versions`  *(v0.5)*
- [ ] `user_profiles.get / update / list`  *(v0.5)*
- [ ] `beta.models.list / get`  *(v0.5)*

## Bedrock provider (`feature = "bedrock"`)

- [ ] AWS SigV4 signing for messages endpoint  *(v0.4)*
- [ ] EventStream protocol for streaming responses  *(v0.4)*
- [ ] Default credential chain integration  *(v0.4)*
- [ ] Static credentials  *(v0.4)*
- [ ] Bearer token auth  *(v0.4)*
- [ ] Custom endpoint override  *(v0.4)*

## Vertex AI provider (`feature = "vertex"`)

- [ ] Google Application Default Credentials  *(v0.4)*
- [ ] Explicit credentials  *(v0.4)*
- [ ] URL path rewriting (`:rawPredict` / `:streamRawPredict` / `:countTokens`)  *(v0.4)*
- [ ] `anthropic_version: vertex-2023-10-16` injection  *(v0.4)*

## AWS external gateway

- [ ] `aws::Client` with workspace ID routing  *(v0.4)*
- [ ] Env vars `ANTHROPIC_AWS_API_KEY`, `ANTHROPIC_AWS_BASE_URL`, `ANTHROPIC_AWS_WORKSPACE_ID`  *(v0.4)*
- [ ] Credential precedence chain  *(v0.4)*

## Pagination

- [ ] Cursor-based `Page<T>` (after_id / before_id)  *(v0.2)*
- [ ] Token-based `TokenPage<T>`  *(v0.3)*
- [ ] Offset-based `PageCursor<T>`  *(v0.3)*
- [ ] `AutoPager<T>` async iterator for each variant  *(v0.2)*

## JSONL streaming

- [ ] Generic `JsonlStream<T>` for batch results  *(v0.3)*

## Cross-cutting

- [x] `request-id` surfaced on every `Error::Api`  *(v0.1)*
- [x] Per-request timeout via `ClientBuilder::timeout` (server side; retry not wired yet)  *(v0.1)*
- [x] Custom `reqwest::Client` injection  *(v0.1)*
- [ ] Retries with exponential backoff + jitter (Retry-After honoured)  *(v0.2)*
- [ ] Custom middleware hook (request/response interceptor)  *(v0.3)*
- [ ] `anthropic-beta` header opt-in helper  *(v0.2)*
- [ ] `tracing` instrumentation on every request  *(v0.2)*
- [ ] OpenTelemetry feature flag  *(v0.4)*

## Auth

- [x] `ANTHROPIC_API_KEY` from env  *(v0.1)*
- [x] `ANTHROPIC_BASE_URL` from env  *(v0.1)*
- [ ] `ANTHROPIC_AUTH_TOKEN` (bearer alternative)  *(v0.3)*
- [ ] `ANTHROPIC_CUSTOM_HEADERS` newline-separated parsing  *(v0.3)*
- [ ] `ANTHROPIC_PROFILE` config-file based auth  *(v0.4)*
- [ ] Workload identity federation (`FederationOptions`)  *(v0.4)*
- [ ] `ANTHROPIC_IDENTITY_TOKEN_FILE` rotation  *(v0.4)*

## Beta header registry

- [ ] `message-batches-2024-09-24`  *(v0.3)*
- [ ] `prompt-caching-2024-07-31`  *(v0.2)*
- [ ] `pdfs-2024-09-25`  *(v0.2)*
- [ ] `token-counting-2024-11-01`  *(v0.2)*
- [ ] `token-efficient-tools-2025-02-19`  *(v0.2)*
- [ ] `output-128k-2025-02-19`  *(v0.2)*
- [ ] `output-300k-2026-03-24`  *(v0.2)*
- [ ] `files-api-2025-04-14`  *(v0.3)*
- [ ] `mcp-client-2025-04-04`  *(v0.5)*
- [ ] `mcp-client-2025-11-20`  *(v0.5)*
- [ ] `dev-full-thinking-2025-05-14`  *(v0.2)*
- [ ] `interleaved-thinking-2025-05-14`  *(v0.2)*
- [ ] `code-execution-2025-05-22`  *(v0.3)*
- [ ] `extended-cache-ttl-2025-04-11`  *(v0.2)*
- [ ] `context-1m-2025-08-07`  *(v0.2)*
- [ ] `context-management-2025-06-27`  *(v0.5)*
- [ ] `skills-2025-10-02`  *(v0.5)*
- [ ] `fast-mode-2026-02-01`  *(v0.2)*
- [ ] `user-profiles-2026-03-24`  *(v0.5)*
- [ ] `computer-use-2024-10-22` / `computer-use-2025-01-24`  *(v0.3)*

## Error types

- [x] Top-level `Error` enum (`Http`, `Api`, `Serde`, `Config`)  *(v0.1)*
- [x] `ErrorType` parsed for: invalid_request, authentication, billing, permission, not_found, rate_limit, timeout, api, overloaded, gateway_timeout  *(v0.1)*
- [x] Forward-compatible `ErrorType::Unknown`  *(v0.1)*
- [ ] `Error::Stream` variant for SSE failures  *(v0.2)*
- [ ] Beta error union (`BetaInvalidRequestError`, etc.) when beta endpoints land  *(v0.5)*

## Examples (port from `anthropic-sdk-go/examples/`)

- [x] `basic_message` (port of `examples/message`)  *(v0.1)*
- [x] `multi_turn` (no Go equivalent — added because requested)  *(v0.1)*
- [ ] `message-streaming`  *(v0.2)*
- [ ] `tools` + `tools-streaming` + `tools-streaming-jsonschema`  *(v0.2)*
- [ ] `tool-runner` + `tool-runner-streaming`  *(v0.2)*
- [ ] `multimodal`  *(v0.2)*
- [ ] `structured-outputs` (raw + typed + streaming)  *(v0.3)*
- [ ] `file-upload`  *(v0.3)*
- [ ] `agents` + `agents-comprehensive` + `agents-with-files`  *(v0.5)*
- [ ] `bedrock` + `bedrock-streaming` + `bedrock-bearer-token`  *(v0.4)*
- [ ] `vertex` + `vertex-streaming`  *(v0.4)*
- [ ] `message-mcp-streaming`  *(v0.5)*
- [ ] `config-file-auth`  *(v0.4)*
- [ ] `workload-identity-env`  *(v0.4)*
- [ ] `mock_server` (in-process mock for offline development)  *(v0.2)*

## Tooling & CI

- [x] `cargo fmt --check`  *(v0.1)*
- [x] `cargo clippy --all-targets --all-features -- -D warnings`  *(v0.1)*
- [x] `cargo test` (no API key, wiremock-driven)  *(v0.1)*
- [x] `cargo test --features live-tests` (real API)  *(v0.1)*
- [x] `cargo doc --no-deps` (warnings as errors)  *(v0.1)*
- [ ] `cargo audit`  *(v0.2)*
- [ ] `cargo deny check`  *(v0.2)*
- [ ] `cargo machete`  *(v0.2)*
- [ ] GitHub Actions workflow (matrix: `--no-default-features`, default, `--features bedrock`, `--all-features`)  *(v0.2)*
- [ ] Publish to crates.io  *(v0.2)*
