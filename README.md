## hookman

Add hooks to your Git repository using a TOML file.

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

Once your commands are in place, run:

```bash
hookman build
```

To list all installed hooks, run:

```bash
hookman list
```

And, to remove/clean all hooks:

```bash
hookman clean
```

## Installation

Install using `cargo`:

```bash
cargo install hookman
```

## License

Licensed under the MIT License. Please check [LICENSE] for more information.
