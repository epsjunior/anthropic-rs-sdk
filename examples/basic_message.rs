//! Single-turn happy path.
//!
//! Run with:
//! ```text
//! ANTHROPIC_API_KEY=sk-ant-... cargo run --example basic_message
//! ```

use anthropic::types::{InputMessage, MessageCreateParams, model};
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let client = anthropic::Client::from_env().context("building Client from environment")?;

    let response = client
        .messages()
        .create(
            MessageCreateParams::builder()
                .model(model::CLAUDE_HAIKU_4_5)
                .max_tokens(256)
                .messages(vec![InputMessage::user(
                    "In one short sentence, say hello as a Rust crate would.",
                )])
                .build(),
        )
        .await
        .context("messages.create")?;

    println!("--- response ---");
    println!("{}", response.text());
    println!("--- usage ---");
    println!(
        "input={} output={}",
        response.usage.input_tokens, response.usage.output_tokens
    );
    println!("stop_reason={:?}", response.stop_reason);

    Ok(())
}
