mod cli;
mod client;
mod commands;
mod config;
mod llm_help;
mod output;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};
#[cfg(unix)]
use cli::SkillAction;
use client::MetabaseClient;

fn main() -> Result<()> {
    if std::env::args().any(|a| a == "--llm") {
        llm_help::print_llm_help();
        return Ok(());
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Config => commands::config_cmd::run(),
        #[cfg(unix)]
        Commands::Skill { action } => match action {
            SkillAction::Add { force } => commands::skill::add(force),
            SkillAction::Update => commands::skill::update(),
            SkillAction::Remove => commands::skill::remove(),
            SkillAction::Status => commands::skill::status(),
        },
        Commands::Databases => {
            let client = connect()?;
            commands::databases::run(&client)
        }
        Commands::Tables { database } => {
            let client = connect()?;
            commands::tables::run(&client, &database)
        }
        Commands::Fields { database, table } => {
            let client = connect()?;
            commands::fields::run(&client, &database, &table)
        }
        Commands::Query { database, sql, json, csv } => {
            let client = connect()?;
            commands::query::run(&client, &database, &sql, json, csv)
        }
    }
}

fn connect() -> Result<MetabaseClient> {
    let cfg = config::load()?;
    MetabaseClient::new(&cfg)
}
