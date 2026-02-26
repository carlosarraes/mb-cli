mod cli;
mod client;
mod commands;
mod config;
mod output;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};
use client::MetabaseClient;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Commands::Config = &cli.command {
        return commands::config_cmd::run();
    }

    let cfg = config::load()?;
    let client = MetabaseClient::new(&cfg)?;

    match cli.command {
        Commands::Config => unreachable!(),
        Commands::Databases => commands::databases::run(&client),
        Commands::Tables { database } => commands::tables::run(&client, &database),
        Commands::Fields { database, table } => commands::fields::run(&client, &database, &table),
        Commands::Query { database, sql, json, csv } => {
            commands::query::run(&client, &database, &sql, json, csv)
        }
    }
}
