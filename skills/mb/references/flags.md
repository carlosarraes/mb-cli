# mb Command Reference

## mb config

Set Metabase URL and authentication. Interactive prompts — no flags.

## mb databases

List all databases. No flags.

Output columns: ID, Name, Engine.

## mb tables \<DATABASE\>

List tables in a database.

| Argument | Description |
|----------|-------------|
| DATABASE | Database name (case-insensitive) or numeric ID |

Output columns: ID, Schema, Name.

## mb fields \<DATABASE\> \<TABLE\>

List fields of a table with types.

| Argument | Description |
|----------|-------------|
| DATABASE | Database name or ID |
| TABLE    | Table name (case-insensitive) or numeric ID |

Output columns: ID, Name, DB Type, Base Type, Semantic Type.

## mb query \<DATABASE\> \<SQL\>

Run a native SQL query.

| Argument | Description |
|----------|-------------|
| DATABASE | Database name or ID |
| SQL      | SQL query string (quote it) |

| Flag     | Description |
|----------|-------------|
| --json   | Output as JSON |
| --csv    | Output as CSV |

Default output is a pretty-printed table.

## mb collections

List all collections.

| Flag   | Description |
|--------|-------------|
| --json | Output as JSON |

Output columns: ID, Name, Parent ID, Personal.

## mb questions

List saved questions (cards).

| Flag                    | Description |
|-------------------------|-------------|
| --collection \<ID\|NAME\> | Filter by collection ID or name |
| --search \<TEXT\>         | Case-insensitive substring search on name |
| --archived              | Include archived questions (excluded by default) |
| --json                  | Output as JSON |

Output columns: ID, Name, Query Type, DB ID, Collection ID, Archived, Updated At.

## mb question \<ID_OR_NAME\>

Inspect a saved question (card). Resolves by numeric ID or case-insensitive exact name.

| Argument    | Description |
|-------------|-------------|
| ID_OR_NAME  | Question ID or exact name |

| Flag      | Description |
|-----------|-------------|
| --inspect | Pretty-print dataset_query JSON |
| --sql     | Print native SQL query (errors if query-builder question) |
| --json    | Output full card as JSON |

Default output: human-friendly summary (id, name, query_type, database_id, collection_id, archived, updated_at, description).

## Global

| Flag      | Description |
|-----------|-------------|
| --help    | Show help |
| --version | Show version |
| --llm     | Show LLM-optimized query guidance |
