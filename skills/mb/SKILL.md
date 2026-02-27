---
name: mb
description: Query Metabase databases, inspect schemas, and run SQL from the terminal using the `mb` CLI. Use this skill whenever the user mentions Metabase, wants to query a database through Metabase, inspect table schemas, list database fields, check column types, or run SQL queries against Metabase-connected databases. Trigger on phrases like "query metabase", "check the database", "what tables are in", "show me the fields", "run this SQL", "inspect schema", "list databases", "mb query", "mb tables", "mb fields", or any data exploration task involving Metabase. This skill is essential for any Metabase data exploration workflow — when in doubt about whether to use it, use it.
---

# mb — Metabase CLI

`mb` is a CLI for querying Metabase databases. It wraps the Metabase REST API to provide schema inspection and native SQL querying from the terminal.

## Prerequisites

The user must have `mb` installed and configured. Check with:

```bash
mb --help
mb databases  # will fail if not configured
```

If not configured, run `mb config` which prompts for Metabase URL and authentication (API key or session token from browser cookie).

## Data Exploration Workflow

Follow this sequence. This is the standard way to explore an unfamiliar database.

### 1. Find the Database

```bash
# List all connected databases
mb databases
```

Output shows ID, Name, and Engine. Use either name or ID in subsequent commands.

### 2. List Tables

```bash
# By name (case-insensitive)
mb tables ZapSign

# By ID
mb tables 2
```

Output shows ID, Schema, and Name for every table.

### 3. Inspect Fields

```bash
# See column names, types, and semantic info
mb fields ZapSign users
```

Output shows ID, Name, DB Type, Base Type, and Semantic Type. Use this to understand what columns are available before writing queries.

### 4. Query

```bash
# Pretty table output (default)
mb query ZapSign "SELECT * FROM users LIMIT 10"

# JSON output for programmatic analysis
mb query ZapSign "SELECT * FROM users LIMIT 10" --json

# CSV for piping or export
mb query ZapSign "SELECT * FROM users LIMIT 10" --csv
```

## Common SQL Patterns

```bash
# Count rows
mb query mydb "SELECT count(*) FROM orders"

# Filter
mb query mydb "SELECT * FROM users WHERE created_at > '2025-01-01' LIMIT 20"

# Aggregations
mb query mydb "SELECT status, count(*) FROM orders GROUP BY status"

# Joins
mb query mydb "SELECT u.name, count(o.id) FROM users u JOIN orders o ON o.user_id = u.id GROUP BY u.name ORDER BY count DESC LIMIT 10"

# Distinct values (useful for understanding enums/statuses)
mb query mydb "SELECT DISTINCT status FROM orders"
```

## Key Patterns

- **Always inspect before querying** — run `mb fields` to see column names and types before writing SQL
- **Use `--json`** when you need to parse or analyze results programmatically
- **Use `--csv`** for piping to other tools or exporting data
- **Name or ID** — all database/table arguments accept either a name (case-insensitive) or numeric ID
- **Start with LIMIT** — always add `LIMIT` to exploratory queries to avoid pulling massive result sets

## Flag Reference

For the full list of flags on each command, read `references/flags.md` in this skill.
