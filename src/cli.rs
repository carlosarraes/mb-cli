use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mb", about = "Metabase CLI", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Config,
    Databases,
    Tables {
        database: String,
    },
    Fields {
        database: String,
        table: String,
    },
    Query {
        database: String,
        sql: String,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        csv: bool,
    },
}
