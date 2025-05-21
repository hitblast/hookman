# hookman

Add hooks to your Git repository using a TOML file.

> [!IMPORTANT]
> If you like this project, consider starring! ⭐ It's free, and it always motivates me to make more of such projects. :D

## Table of Contents

- [Usage](#usage)
- [Installation](#installation)
- [Contributing](#contributing)
- [License](#license)

## Usage

Using hookman is pretty straightforward.

When inside the directory of a Git repository,
create a new `hookman.toml` with the following structure:

```toml
# structure:
[hook.<event>]  # here "event" is the event for running the hook
run = "<command>"  # and here is your actual shell command

# example:
[hook.pre-commit]
run = "pip install -U -r requirements.txt && pip list > requirements.txt"
```

1. To put your hooks into action, run:

```bash
hookman build
```

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
