# hookman

[![Rust Tests](https://github.com/hitblast/hookman/actions/workflows/tests.yml/badge.svg)](https://github.com/hitblast/hookman/actions/workflows/tests.yml)
[![Release Builds](https://github.com/hitblast/hookman/actions/workflows/release.yml/badge.svg)](https://github.com/hitblast/hookman/actions/workflows/release.yml)

Add hooks to your Git repository using a TOML file.

> [!IMPORTANT]
> If you like this project, consider starring! ⭐ It's free, and it always motivates me to make more of such projects. :D

## Table of Contents

- [Key Features](#key-features)
- [Usage](#usage)
- [Installation](#installation)
- [Contributing](#contributing)
- [License](#license)

## Key Features

- Automates the creation of multiple git hooks with just one TOML file.
- No symlinks, no additional `chmod +x` commands or perm-handling is needed. `hookman` does it all for you.
- Hooks event validation so that you don't accidentally write a wrong hook type.
- Tiny & fast; Made using Rust.

## Usage

The usage is pretty straightforward.

When inside the directory of a Git repository, create a new `hookman.toml` file with the following structure:

```toml
[hook.<event>]  # the hook type/event (pre-commit, update etc.)
run = "<command>"  # you can either have a `run` field with the command itself
script = "<script path>"  # or, your personal script inside the directory
```

For example:
```toml
[hook.pre-commit]
run = "pip install -U -r requirements.txt && pip list > requirements.txt"

[hook.pre-push]
script = "scripts/bundle-app.sh"
```

---

1. To put your hooks into action, run:

```bash
hookman build

# or use this command to use the current shell for hook execution later on
hookman build --use-current-shell
```

With this command, `hookman` handles all script permissions, relocation and other mundane tasks without you ever having to touch it.

2. To list all installed hooks, run:

```bash
hookman list
```

3. To list all possible events a hook can be attached to, run:

```bash
hookman list-events
```

4. And, to remove/clean all hooks:

```bash
hookman clean

# use --all/a to delete the directory + stale hooks
hookman clean --all
```

## Installation

Install using `cargo`:

```bash
cargo install hookman
```

Or, you can set it up globally using `mise`:

```bash
# Note: This will compile the binary for your system.
mise use -g cargo:hookman
```

For macOS, you can install using [Homebrew](https://brew.sh/):
```bash
brew install hitblast/tap/hookman
```

### Manual Installation

If your platform isn't enlisted here, you can opt for the [compressed binary downloads](https://github.com/hitblast/hookman/releases) in the GitHub Releases section of the repository.

Note than on devices running macOS, you'll have to remove the quarantine attribute from the binary:

```bash
xattr -d com.apple.quarantine bin/hookman  # inside extracted zip
```

## Contributing

hookman is a very tiny project for those who'd like to use an extremely minimal setup for managing git hooks, so I don't think there is a need to follow a mandatory set of rules for contribution. Anyhow, pull requests, and new issues regarding feature suggestions, bug fixes or new ideas are always welcome!

## License

Licensed under the MIT License. Please check [LICENSE](./LICENSE) for more information.
