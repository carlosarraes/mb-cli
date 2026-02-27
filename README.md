# mb

A CLI for querying Metabase databases.

## Install

```bash
just build
```

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

Database and table args accept **name or ID**.

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
