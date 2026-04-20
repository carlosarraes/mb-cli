pub fn print_llm_help() {
    print!(
        r#"# mb — Metabase CLI

Query Metabase databases from the command line. Supports schema inspection and native SQL.

## Setup

```bash
mb config  # prompts for URL + API key or session token
```

Config stored at `~/.config/mb/config.toml`.

## Schema Inspection

```bash
mb databases                    # list all databases (ID, name, engine)
mb tables <database>            # list tables (ID, schema, name)
mb fields <database> <table>    # list fields (ID, name, DB type, base type, semantic type)
```

Database/table args accept **name or ID**. Names are case-insensitive.

## Querying

```bash
mb query <database> "SELECT * FROM users LIMIT 10"        # pretty table
mb query <database> "SELECT * FROM users LIMIT 10" --json  # JSON output
mb query <database> "SELECT * FROM users LIMIT 10" --csv   # CSV output
```

Queries run as native SQL via `POST /api/dataset`.

## Common Query Patterns

```sql
-- Count rows
mb query mydb "SELECT count(*) FROM orders"

-- Filter with WHERE
mb query mydb "SELECT * FROM users WHERE created_at > '2025-01-01' LIMIT 20"

-- Aggregations
mb query mydb "SELECT status, count(*) FROM orders GROUP BY status"

-- Joins
mb query mydb "SELECT u.name, count(o.id) FROM users u JOIN orders o ON o.user_id = u.id GROUP BY u.name ORDER BY count DESC LIMIT 10"
```

## Saved Questions (Cards)

```bash
mb collections                              # list all collections
mb collections --json                       # raw JSON
mb questions                                # list all saved questions
mb questions --collection 400               # filter by collection ID
mb questions --collection "Finance"         # filter by collection name
mb questions --search "monthly report"      # substring search on name
mb questions --archived                     # include archived questions
mb question 4707                            # human-friendly summary
mb question 4707 --inspect                  # pretty-print dataset_query JSON
mb question 4707 --sql                      # print native SQL (if native question)
mb question 4707 --json                     # full card JSON
mb question "My Question Name"              # resolve by exact name (case-insensitive)
```

Use `mb question <id> --sql` to extract the SQL from a saved native question.
Use `mb question <id> --inspect` to see the full query definition (works for both native and query-builder questions).

## Workflow: Explore Then Query

```bash
mb databases                          # find the database
mb tables mydb                        # find the table
mb fields mydb users                  # see column names and types
mb query mydb "SELECT * FROM users LIMIT 5"  # sample data
```

## Auth Methods

- **API key**: `x-api-key` header (requires Metabase admin)
- **Session token**: `X-Metabase-Session` header (from browser cookie, any user)
"#
    );
}
