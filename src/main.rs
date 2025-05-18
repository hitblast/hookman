#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use anyhow::{Context, Result, bail};
use clap::Parser;
use hookman::{Command, Opt};
use serde::Deserialize;

/// Read a hookman.toml of the form:
///   [hook.<hook-type>]
///   run = "…"
/// Section names must be valid Git hook types (e.g., pre-commit, post-merge)
#[derive(Deserialize)]
struct Config {
    hook: HashMap<String, Hook>,
}

#[derive(Deserialize)]
struct Hook {
    run: String,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    match opt.command {
        Command::Build => build_hooks(&opt.config)?,
        Command::List => list_hooks(&opt.config)?,
        Command::Clean => clean_hooks(&opt.config)?,
    }

    Ok(())
}

/// Remove hook scripts defined in the config
fn clean_hooks(config_path: &PathBuf) -> Result<()> {
    // parse toml
    let toml_str = fs::read_to_string(config_path)
        .with_context(|| format!("reading config `{}`", config_path.display()))?;
    let cfg: Config = toml::from_str(&toml_str).context("parsing hookman.toml")?;

    // find git root
    let git_root = find_git_root().context("not inside a Git repository")?;
    let hooks_dir = git_root.join(".git").join("hooks");

    for hook_name in cfg.hook.keys() {
        let hook_path = hooks_dir.join(hook_name);
        if hook_path.exists() {
            fs::remove_file(&hook_path)
                .with_context(|| format!("removing hook file `{}`", hook_name))?;
            println!("Removed hook `{}`", hook_name);
        } else {
            println!("No hook `{}` to remove, skipping", hook_name);
        }
    }
    Ok(())
}

fn build_hooks(config_path: &PathBuf) -> Result<()> {
    // parse toml
    let toml_str = fs::read_to_string(config_path)
        .with_context(|| format!("reading config `{}`", config_path.display()))?;
    let cfg: Config = toml::from_str(&toml_str).context("parsing hookman.toml")?;

    // find git root
    let git_root = find_git_root().context("not inside a Git repository")?;
    let hooks_dir = git_root.join(".git").join("hooks");
    fs::create_dir_all(&hooks_dir).context("creating .git/hooks directory")?;

    // validate supported hook types (map keys)
    const VALID_HOOKS: &[&str] = &[
        "applypatch-msg",
        "pre-applypatch",
        "post-applypatch",
        "pre-commit",
        "pre-merge-commit",
        "prepare-commit-msg",
        "commit-msg",
        "post-commit",
        "pre-rebase",
        "post-checkout",
        "post-merge",
        "pre-push",
        "pre-receive",
        "update",
        "post-receive",
        "post-update",
        "push-to-checkout",
        "pre-auto-gc",
        "post-rewrite",
        "sendemail-validate",
        "fsmonitor-watchman",
        "proc-receive",
    ];

    for (hook_name, hook) in cfg.hook {
        if !VALID_HOOKS.contains(&hook_name.as_str()) {
            bail!("unsupported hook type `{}`", hook_name);
        }
        let dest = hooks_dir.join(&hook_name);

        if dest.exists() {
            println!("Overwriting hook `{}`", hook_name);
        }

        let mut file = fs::File::create(&dest)
            .with_context(|| format!("creating hook file `{}`", dest.display()))?;

        // shebang + set -e + the user’s command
        writeln!(file, "#!/usr/bin/env bash")?;
        writeln!(file, "set -e")?;
        writeln!(file, "{}", hook.run)?;

        // make the hook executable on Unix; on Windows skip
        #[cfg(unix)]
        {
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&dest, perms)?;
        }

        println!("Installed hook `{}`", hook_name);
    }

    Ok(())
}

fn list_hooks(config_path: &PathBuf) -> Result<()> {
    // parse toml
    let toml_str = fs::read_to_string(config_path)
        .with_context(|| format!("reading config `{}`", config_path.display()))?;
    let cfg: Config = toml::from_str(&toml_str).context("parsing hookman.toml")?;

    if cfg.hook.is_empty() {
        println!("No hooks defined in {}", config_path.display());
        return Ok(());
    }

    println!("Hooks defined in {}:", config_path.display());
    let mut hooks: Vec<_> = cfg.hook.keys().collect();
    hooks.sort();
    for hook in hooks {
        println!("- {}", hook);
    }

    Ok(())
}

/// Climb up until we hit a `.git` directory
fn find_git_root() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if dir.join(".git").is_dir() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}
