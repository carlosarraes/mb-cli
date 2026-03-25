---
name: mb
description: Query Metabase databases, inspect schemas, run SQL, and explore saved questions from the terminal using the `mb` CLI. Use this skill whenever the user mentions Metabase, wants to query a database through Metabase, inspect table schemas, list database fields, check column types, run SQL queries, browse collections, find saved questions/cards, or extract SQL from existing questions. Trigger on phrases like "query metabase", "check the database", "what tables are in", "show me the fields", "run this SQL", "inspect schema", "list databases", "mb query", "mb tables", "mb fields", "mb collections", "mb questions", "mb question", "saved questions", "find the question", "what questions are in", "extract SQL from question", or any data exploration task involving Metabase. This skill is essential for any Metabase data exploration workflow — when in doubt about whether to use it, use it.
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

## Saved Questions (Cards)

Use these commands to browse and inspect saved Metabase questions without writing Python scripts or using curl.

### Browse Collections

```bash
mb collections                              # list all collections
mb collections --json                       # raw JSON output
```

### Find Questions

```bash
mb questions                                # list all saved questions
mb questions --collection 400               # filter by collection ID
mb questions --collection "Pagamentos"      # filter by collection name
mb questions --search "FASE 1"              # substring search on name
mb questions --collection "Pagamentos" --search "FASE 1"  # combine filters
mb questions --archived                     # include archived questions
mb questions --json                         # raw JSON output
```

### Inspect a Question

```bash
mb question 4707                            # human-friendly summary
mb question 4707 --inspect                  # pretty-print dataset_query JSON
mb question 4707 --sql                      # print native SQL
mb question 4707 --json                     # full card JSON
mb question "My Question Name"              # resolve by exact name
```

- Use `--sql` to extract the SQL from a native question (fails gracefully for query-builder questions)
- Use `--inspect` to see the full query definition regardless of question type

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
- **Check saved questions first** — use `mb questions --search` to find existing questions before writing new SQL
- **Extract SQL from questions** — use `mb question <id> --sql` to get the SQL from a native question, then modify it

## Flag Reference

For the full list of flags on each command, read `references/flags.md` in this skill.
