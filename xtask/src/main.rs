use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use clap_mangen::Man;
use hookman::Opt;
use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate manpage for hookman
    Manpage {
        /// Output directory for the manpage
        #[arg(short, long, default_value = "man/man1")]
        dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Manpage { dir } => {
            generate_manpage(dir)?;
        }
    }

    Ok(())
}

fn generate_manpage(dir: PathBuf) -> Result<()> {
    // Create the output directory
    fs::create_dir_all(&dir).context("Failed to create output directory")?;

    // Generate the manpage
    let file_path = dir.join("hookman.1");
    let mut file = File::create(&file_path).context("Failed to create manpage file")?;

    let cmd = Opt::command();

    Man::new(cmd)
        .render(&mut file)
        .context("Failed to render manpage")?;

    println!("Manpage generated at: {}", file_path.display());

    Ok(())
}
