use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "mb",
    about = "Query Metabase databases from the command line",
    version,
    after_help = "EXAMPLES\n  $ mb config\n  $ mb databases\n  $ mb tables ZapSign\n  $ mb fields ZapSign users\n  $ mb query ZapSign \"SELECT * FROM users LIMIT 10\"\n  $ mb query 2 \"SELECT count(*) FROM orders\" --csv\n\nLEARN MORE\n  Run 'mb --llm' for LLM-optimized query guidance and examples."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Set Metabase URL and authentication")]
    Config,

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
}
