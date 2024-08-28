mod cli;

use clap::{Parser, Subcommand, ValueEnum};
use log::{info, LevelFilter};
use std::sync::{Arc, Mutex};

// Import the generate module with an alias to avoid name conflict
use crate::cli::generate::{Generate as GenerateCLI, run_generate};

#[derive(Parser)]
#[command(
    name = "RsGen",
    version = "0.0.1",
    author = "Chris Jackett",
    about = "A Rust Generative AI Co-pilot",
    arg_required_else_help(true)
)]
struct RsGen {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(
        long,
        value_enum,
        default_value = "info",
        help = "Logging level"
    )]
    level: LogLevel,

    #[arg(
        long,
        help = "Show the prompt being sent to the LLM."
    )]
    show: bool,
}

#[derive(Subcommand)]
enum Commands {
    Convert(Convert),
    Explain(Explain),
    Generate(GenerateCLI),
    Git(Git),
    Refactor(Refactor),
    Resolve(Resolve),
    Review(Review),
}

#[derive(ValueEnum, Clone)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}


#[derive(Parser)]
#[command(
    name = "convert",
    about = "Convert various file types into Rust code.",
    arg_required_else_help(true)
)]
struct Convert;

#[derive(Parser)]
#[command(
    name = "explain",
    about = "Provide detailed explanations for the Rust codebase, module, struct, enum or function.",
    arg_required_else_help(true)
)]
struct Explain;

#[derive(Parser)]
#[command(
    name = "git",
    about = "Perform Git operations such as checking staged changes, generating commit messages, or creating pull request messages.",
    arg_required_else_help(true)
)]
struct Git;

#[derive(Parser)]
#[command(
    name = "refactor",
    about = "Refactor Rust code including modules, structs, enums or functions to improve structure and readability.",
    arg_required_else_help(true)
)]
struct Refactor;

#[derive(Parser)]
#[command(
    name = "resolve",
    about = "Resolve issues detected by Rust tools like cargo-audit, clippy, rustfmt and the Rust compiler.",
    arg_required_else_help(true)
)]
struct Resolve;

#[derive(Parser)]
#[command(
    name = "review",
    about = "Review the codebase, modules, structs, enums or functions and suggest improvements.",
    arg_required_else_help(true)
)]
struct Review;



fn main() {
    let args = RsGen::parse();

    // Set the logging level
    let level = match args.level {
        LogLevel::Trace => LevelFilter::Trace,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Off => LevelFilter::Off,
    };

    env_logger::Builder::from_default_env()
        .filter(None, level)
        .init();

    info!("Initialised RsGen CLI v0.0.1");

    // Set up LLM client and context (placeholder)
    let _llm_client = Arc::new(Mutex::new(LLMClient::new()));
    let _show = args.show;

}

// Placeholder struct for LLMClient
struct LLMClient;

impl LLMClient {
    fn new() -> Self {
        // Initialize the LLM client
        LLMClient {}
    }
}
