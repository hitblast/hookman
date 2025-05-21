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
    Build {
        /// Use the shell from the current session for the hooks.
        /// If not set, this will default to /usr/bin/env bash
        #[arg(short, long)]
        use_current_shell: bool,
    },
    /// List all hooks defined in the config
    List,
    /// Delete all hooks defined in the config
    Clean,
    /// List all possible events for running hooks
    ListEvents,
}
