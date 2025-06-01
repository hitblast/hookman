#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{collections::HashMap, env, fs, io::Write, path::PathBuf};

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
    run: Option<String>,
    script: Option<String>,
}

/// A list of supported hook events/types.
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

fn main() -> Result<()> {
    let opt = Opt::parse();

    // If the config file doesn't exist, display a clear error and exit.
    if !opt.config.exists() {
        bail!("config file not found: {}", opt.config.display());
    }
    
    if !opt.ignore_stale {
        let config_content = fs::read_to_string(&opt.config)
            .with_context(|| format!("reading config `{}`", opt.config.display()))?;
        let cfg: Config = toml::from_str(&config_content)
            .context("parsing hookman.toml")?;
        warn_stale_hooks(&cfg);
    }

    match opt.command {
        Command::Build { use_current_shell } => build_hooks(use_current_shell, &opt.config)?,
        Command::List => list_hooks(&opt.config)?,
        Command::ListEvents => list_events(),
        Command::Clean => clean_hooks(&opt.config)?,
    }

    Ok(())
}

/*

Helper functions

*/

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

/*

Command-specific functions

*/

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
            println!("removed hook `{}`", hook_name);
        } else {
            println!("no hook `{}` to remove, skipping", hook_name);
        }
    }
    Ok(())
}

fn build_hooks(use_current_shell: bool, config_path: &PathBuf) -> Result<()> {
    // parse toml
    let toml_str = fs::read_to_string(config_path)
        .with_context(|| format!("reading config `{}`", config_path.display()))?;
    let cfg: Config = toml::from_str(&toml_str).context("parsing hookman.toml")?;

    // find git root
    let git_root = find_git_root().context("not inside a git repository")?;
    let hooks_dir = git_root.join(".git").join("hooks");
    fs::create_dir_all(&hooks_dir).context("creating .git/hooks directory")?;

    for (hook_name, hook) in cfg.hook {
        let use_run = hook.run.is_some();
        let use_script = hook.script.is_some();

        if !VALID_HOOKS.contains(&hook_name.as_str()) {
            bail!("unsupported hook type `{}`", hook_name)
        } else if use_run && use_script {
            bail!(
                "hook {}: either `run` or `script` can be assigned at a time.",
                hook_name
            )
        } else if !(use_run || use_script) {
            bail!(
                "hook {}: you must use either `run` or `script` in the definition.",
                hook_name
            )
        }

        // set destination file
        let dest = hooks_dir.join(&hook_name);
        if dest.exists() {
            println!("overwriting hook `{}`", hook_name);
        }

        let mut file = fs::File::create(&dest)
            .with_context(|| format!("creating hook file `{}`", dest.display()))?;

        // if the `run` tag is used, copy its contents over to a new script
        if use_run {
            // determine the default shell
            let default_env = String::from("/usr/bin/env bash");

            let shell = if use_current_shell {
                env::var("SHELL").unwrap_or(default_env)
            } else {
                default_env
            };

            // shebang + set -e + the user’s command
            writeln!(file, "#!{}", shell)?;
            writeln!(file, "set -e")?;
            writeln!(file, "{}", hook.run.unwrap())?;
        }
        // if the `script` tag is used, copy the contents of the script itself
        else if use_script {
            let path = hook.script.unwrap();

            if !fs::exists(&path).unwrap() {
                bail!("hook {}: script path doesn't exist", hook_name)
            }

            let data = fs::read_to_string(path).unwrap();
            write!(file, "{}", data)?;
        }

        // make the hook executable on Unix; on Windows, skip this step
        #[cfg(unix)]
        {
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&dest, perms)?;
        }

        println!("installed hook `{}`", hook_name);
    }

    Ok(())
}

fn list_hooks(config_path: &PathBuf) -> Result<()> {
    // parse toml
    let toml_str = fs::read_to_string(config_path)
        .with_context(|| format!("reading config `{}`", config_path.display()))?;
    let cfg: Config = toml::from_str(&toml_str).context("parsing hookman.toml")?;

    if cfg.hook.is_empty() {
        println!("no hooks defined in {}", config_path.display());
        return Ok(());
    }

    println!("hooks defined in {}:", config_path.display());
    let mut hooks: Vec<_> = cfg.hook.keys().collect();
    hooks.sort();
    for hook in hooks {
        println!("- {}", hook);
    }

    if let Some(git_root) = find_git_root() {
        let hooks_dir = git_root.join(".git").join("hooks");
        if let Ok(entries) = fs::read_dir(&hooks_dir) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if !file_name.ends_with(".sample")
                        && VALID_HOOKS.contains(&file_name.as_str())
                        && !cfg.hook.contains_key(&file_name)
                    {
                        println!("warning: stale hook: {}", file_name);
                    }
                }
            }
        }
    }
    Ok(())
}

fn list_events() {
    for entry in VALID_HOOKS {
        println!("{}", entry);
    }
}

fn warn_stale_hooks(cfg: &Config) {
    if let Some(git_root) = find_git_root() {
        let hooks_dir = git_root.join(".git").join("hooks");
        if let Ok(entries) = fs::read_dir(&hooks_dir) {
            let tracked: Vec<&str> = cfg.hook.keys().map(|s| s.as_str()).collect();
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if !file_name.ends_with(".sample")
                        && VALID_HOOKS.contains(&file_name.as_str())
                        && !tracked.contains(&file_name.as_str())
                    {
                        println!("Warning: stale hook: {}", file_name);
                    }
                }
            }
        }
    }
}
