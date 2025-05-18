## hooker

Add hooks to your Git repository using a TOML file.

## Usage

Using hooker is pretty straightforward.

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
hooker build
```

## Installation

Install using `cargo`:

```bash
cargo install hooker
```

## License

Licensed under the MIT License. Please check [LICENSE] for more information.
