//! Multi-turn conversation demonstrating two model variants.
//!
//! Sends `[user, assistant, user]` to Sonnet, then forwards the entire history
//! plus the new exchange to Haiku — exercises both the conversation array
//! shape and the [`Model`] constants.
//!
//! Run with:
//! ```text
//! ANTHROPIC_API_KEY=sk-ant-... cargo run --example multi_turn
//! ```

use anthropic::types::{InputMessage, Message, MessageCreateParams, Model, model};
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

    let mut history: Vec<InputMessage> = vec![
        InputMessage::user("My name is Edinaldo and I work on payment infrastructure."),
        InputMessage::assistant(
            "Nice to meet you, Edinaldo. How can I help with your payment infrastructure work today?",
        ),
        InputMessage::user("What name did I just give you?"),
    ];

    let first = ask(&client, model::CLAUDE_SONNET_4_6, history.clone()).await?;
    println!("--- Sonnet 4.6 reply ---");
    println!("{}", first.text());

    // Append the assistant's reply, then ask Haiku to continue the conversation.
    history.push(InputMessage::assistant(first.text()));
    history.push(InputMessage::user(
        "Now translate your previous answer into Brazilian Portuguese.",
    ));

    let second = ask(&client, model::CLAUDE_HAIKU_4_5, history).await?;
    println!("\n--- Haiku 4.5 reply ---");
    println!("{}", second.text());

    println!(
        "\nTotals: sonnet input={} output={} | haiku input={} output={}",
        first.usage.input_tokens,
        first.usage.output_tokens,
        second.usage.input_tokens,
        second.usage.output_tokens,
    );
    Ok(())
}

async fn ask(
    client: &anthropic::Client,
    which: Model,
    messages: Vec<InputMessage>,
) -> Result<Message> {
    client
        .messages()
        .create(
            MessageCreateParams::builder()
                .model(which)
                .max_tokens(512)
                .messages(messages)
                .build(),
        )
        .await
        .context("messages.create")
}
