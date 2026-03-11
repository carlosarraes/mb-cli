# mb

A CLI for querying Metabase databases.

## Install

### Quick install (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/carlosarraes/mb-cli/main/install.sh | sh
```

To install a specific version:

```bash
MB_VERSION=v0.1.0 curl -fsSL https://raw.githubusercontent.com/carlosarraes/mb-cli/main/install.sh | sh
```

### Download from GitHub Releases

Pre-built binaries for Linux, macOS, and Windows are available on the
[Releases page](https://github.com/carlosarraes/mb-cli/releases).

### Build from source

```bash
just build
```

Requires [Rust](https://rustup.rs/) and [just](https://github.com/casey/just).
Builds and copies `mb` to `~/.local/bin/`.

## Setup

```bash
mb config
```

Prompts for Metabase URL and auth (API key or session token from browser cookie).

## Commands

| Command | Description |
|---|---|
| `mb config` | Set Metabase URL and authentication |
| `mb databases` | List all databases |
| `mb tables <db>` | List tables in a database |
| `mb fields <db> <table>` | List fields with types |
| `mb query <db> "SQL"` | Run a native SQL query |
| `mb skill add` | Install the AI agent skill and link to detected agents |
| `mb skill update` | Update skill to the latest version |
| `mb skill remove` | Remove skill and unlink from all agents |
| `mb skill status` | Show installation and version info |

Database and table args accept **name or ID**.

The `skill` command manages an AI agent skill that teaches Claude, Cursor, and Codex how to use `mb` effectively. It downloads the skill files and symlinks them into each detected agent's skill directory.

## Examples

```bash
mb databases
mb tables ZapSign
mb fields ZapSign users
mb query ZapSign "SELECT * FROM users LIMIT 10"
mb query ZapSign "SELECT * FROM users" --json
mb query ZapSign "SELECT * FROM users" --csv
```

## LLM Usage

```bash
mb --llm
```

Prints query guidance and examples optimized for LLM agents.
