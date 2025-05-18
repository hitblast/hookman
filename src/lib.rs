use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// CLI options for hookman
#[derive(Parser)]
#[command(
    author,
    version,
    about = "Install or list git hooks from a TOML config"
)]
pub struct Opt {
    /// Path to the config file
    #[arg(short, long, default_value = "hookman.toml")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

/// Available subcommands for hookman
#[derive(Subcommand)]
pub enum Command {
    /// Generate all hooks into .git/hooks
    Build,
    /// List all hooks defined in the config
    List,
    /// Delete all hooks defined in the config
    Clean,
    /// List all possible events for running hooks
    ListEvents,
}
