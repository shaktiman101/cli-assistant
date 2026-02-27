use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use std::process;
use std::time::Duration;

mod config;
mod display;
mod executor;
mod openai;
mod parser;
mod prompt;
mod shell;

use config::Config;
use display::{show_complete_suggestion, show_error, show_info};
use executor::execute_with_confirmation;
use openai::OpenAIClient;
use parser::parse_response;
use prompt::create_prompt;
use shell::{CommandContext, ShellInfo};

/// AI-powered command-line assistant
#[derive(Parser)]
#[command(name = "cli-assistant")]
#[command(about = "AI-powered CLI assistant that fixes commands and explains errors", long_about = None)]
#[command(version)]
struct Cli {
    /// The command that was executed
    #[arg(long)]
    last_cmd: Option<String>,

    /// Exit code of the last command
    #[arg(long)]
    last_exit: Option<i32>,

    /// Error output from the command
    #[arg(long)]
    stderr: Option<String>,

    /// Skip confirmation before executing
    #[arg(long)]
    no_confirm: bool,

    /// Additional context from user
    #[arg(trailing_var_arg = true)]
    user_prompt: Vec<String>,
}

fn run() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Load configuration
    show_info("Initializing...");
    let config = Config::from_env()
        .context("Failed to load configuration")?;

    show_info(&format!("Using model: {}", config.model()));

    // Combine user_prompt vec into a single string
    let user_prompt = if cli.user_prompt.is_empty() {
        None
    } else {
        Some(cli.user_prompt.join(" "))
    };

    // Capture command context
    let context = CommandContext::capture(
        cli.last_cmd,
        cli.last_exit,
        cli.stderr,
        user_prompt,
    ).context("Failed to capture command context")?;

    // Get shell information
    let shell_info = ShellInfo::capture()
        .context("Failed to capture shell information")?;

    // Create prompt
    let prompt_text = create_prompt(&context, &shell_info);

    // Create OpenAI client
    let client = OpenAIClient::new(
        config.api_key().to_string(),
        config.model().to_string(),
        Duration::from_secs(config.timeout()),
    )?;

    // Call OpenAI API
    show_info("Analyzing command...");
    let response = client.chat_completion(&prompt_text)
        .context("Failed to get response from OpenAI API")?;

    // Parse response
    let parsed = parse_response(&response)
        .context("Failed to parse LLM response")?;

    // Display suggestion
    show_complete_suggestion(
        Some(&context.command),
        &parsed.explanation,
        &parsed.command,
    );

    // Execute command
    execute_with_confirmation(&parsed.command, cli.no_confirm)?;

    Ok(())
}

fn main() {
    // Handle Ctrl+C gracefully
    ctrlc::set_handler(|| {
        println!("\n{}", "Cancelled.".yellow());
        process::exit(130);
    }).expect("Error setting Ctrl-C handler");

    // Run the application
    if let Err(e) = run() {
        show_error(&format!("{:#}", e));
        process::exit(1);
    }
}
