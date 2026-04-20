use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "mb",
    about = "Query Metabase databases from the command line",
    version,
    after_help = "EXAMPLES\n  $ mb config\n  $ mb databases\n  $ mb tables analytics\n  $ mb fields analytics users\n  $ mb query analytics \"SELECT * FROM users LIMIT 10\"\n  $ mb query 2 \"SELECT count(*) FROM orders\" --csv\n  $ mb collections\n  $ mb questions --collection 400 --search \"my report\"\n  $ mb question 4707 --sql\n\nLEARN MORE\n  Run 'mb --llm' for LLM-optimized query guidance and examples."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Set Metabase URL and authentication")]
    Config {
        #[arg(long, help = "Only re-enter session cookie (keeps existing URL)")]
        cookie: bool,
    },

    #[command(about = "List all databases")]
    Databases,

    #[command(about = "List tables in a database")]
    Tables {
        #[arg(help = "Database name or ID")]
        database: String,
    },

    #[command(about = "List fields of a table with types")]
    Fields {
        #[arg(help = "Database name or ID")]
        database: String,
        #[arg(help = "Table name or ID")]
        table: String,
    },

    #[command(about = "Run a native SQL query")]
    Query {
        #[arg(help = "Database name or ID")]
        database: String,
        #[arg(help = "SQL query string")]
        sql: String,
        #[arg(long, help = "Output as JSON")]
        json: bool,
        #[arg(long, help = "Output as CSV")]
        csv: bool,
    },

    #[command(about = "List collections")]
    Collections {
        #[arg(long, help = "Output as JSON")]
        json: bool,
    },

    #[command(about = "List saved questions (cards)")]
    Questions {
        #[arg(long, help = "Filter by collection ID or name")]
        collection: Option<String>,
        #[arg(long, help = "Case-insensitive substring search on name")]
        search: Option<String>,
        #[arg(long, help = "Include archived questions")]
        archived: bool,
        #[arg(long, help = "Output as JSON")]
        json: bool,
    },

    #[command(about = "Inspect a saved question (card)")]
    Question {
        #[arg(help = "Question ID or exact name")]
        id_or_name: String,
        #[arg(long, help = "Pretty-print dataset_query JSON")]
        inspect: bool,
        #[arg(long, help = "Print native SQL query")]
        sql: bool,
        #[arg(long, help = "Output full card as JSON")]
        json: bool,
    },

    #[cfg(unix)]
    #[command(about = "Manage AI agent skills")]
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },
}

#[cfg(unix)]
#[derive(Subcommand)]
pub enum SkillAction {
    #[command(about = "Install the mb skill and link to detected AI agents")]
    Add {
        #[arg(long, help = "Overwrite existing non-symlink directories")]
        force: bool,
    },

    #[command(about = "Update to the latest version")]
    Update,

    #[command(about = "Remove skill and unlink from all agents")]
    Remove,

    #[command(about = "Show installation and version info")]
    Status,
}
