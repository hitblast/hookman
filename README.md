## hookman

Add hooks to your Git repository using a TOML file.

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
run = "mise run manpage"
```

1. To put your hooks into action, run:

```bash
hookman build
```

2. To list all installed hooks, run:

```bash
hookman list
```

3. And, to remove/clean all hooks:

```bash
hookman clean
```

## Installation

Install using `cargo`:

```bash
cargo install hookman
```

## Contributing

hookman is a very tiny project for those who'd like to use an extremely minimal setup for managing git hooks, so I don't think there is a need to follow a mandatory set of rules for contribution. Anyhow, pull requests, and new issues regarding feature suggestions, bug fixes or new ideas are always welcome!

## License

Licensed under the MIT License. Please check [LICENSE](./LICENSE) for more information.
