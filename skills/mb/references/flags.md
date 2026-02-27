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

## Global

| Flag      | Description |
|-----------|-------------|
| --help    | Show help |
| --version | Show version |
| --llm     | Show LLM-optimized query guidance |
