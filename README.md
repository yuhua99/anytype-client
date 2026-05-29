# anyclient

A lightweight Rust CLI that wraps Anytype's Local HTTP API.

## What it's for

anyclient lets you control Anytype from the command line or from AI agents. It is a thin wrapper around Anytype's Local HTTP API, making it easy to create, update, search, and manage objects, spaces, properties, files, and more through simple terminal commands.

## Authentication

Anytype can expose its API in two different setups, so anyclient supports two authentication modes:

- `anyclient auth desktop` — For the regular Anytype desktop application, which includes a built-in Local API (defaults to http://127.0.0.1:31009). The app will show a challenge code for you to confirm.

- `anyclient auth headless` — For [anytype-cli](https://github.com/anyproto/anytype-cli) when running it as a headless Anytype server (API defaults to http://127.0.0.1:31012).

Override the server address with `--base-url` or the `ANYTYPE_BASE_URL` environment variable. Provide an API key with `--api-key` / `ANYTYPE_API_KEY` or store it in the config file.

## Quick Start

```bash
# Install directly from git
cargo install --git https://github.com/yuhua99/anytype-client.git

# Authenticate with the desktop app
anyclient auth desktop

# Or authenticate against a headless server
anyclient auth headless --api-key YOUR_API_KEY

# Try some commands
anyclient spaces list
anyclient objects create my-space-id --name "My Note" --type page -o json
```

Use `anyclient --help` and `anyclient <command> -h` for available options and flags. For the full command reference, see docs/cli-commands.md.

## Using with the coding agent

Install the skill so the pi agent can use anyclient:

```bash
bunx skills add git@github.com:yuhua99/anytype-client.git
```

(Use `npx` if `bunx` is not available.)

## Compatibility

anyclient works with:
- The standard Anytype desktop application, or [anytype-cli](https://github.com/anyproto/anytype-cli)

## Configuration

You can configure the base URL and API key using:
- CLI flags (`--base-url`, `--api-key`)
- Environment variables (`ANYTYPE_BASE_URL`, `ANYTYPE_API_KEY`)
- The config file at `~/.anyclient/config.toml`
