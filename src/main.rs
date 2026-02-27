mod cli;
mod client;
mod commands;
mod config;
mod llm_help;
mod output;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands, SkillAction};
use client::MetabaseClient;

fn main() -> Result<()> {
    if std::env::args().any(|a| a == "--llm") {
        llm_help::print_llm_help();
        return Ok(());
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Config => return commands::config_cmd::run(),
        Commands::Skill { action } => return match action {
            SkillAction::Add { force } => commands::skill::add(*force),
            SkillAction::Update => commands::skill::update(),
            SkillAction::Remove => commands::skill::remove(),
            SkillAction::Status => commands::skill::status(),
        },
        _ => {}
    }

    let cfg = config::load()?;
    let client = MetabaseClient::new(&cfg)?;

    match cli.command {
        Commands::Config | Commands::Skill { .. } => unreachable!(),
        Commands::Databases => commands::databases::run(&client),
        Commands::Tables { database } => commands::tables::run(&client, &database),
        Commands::Fields { database, table } => commands::fields::run(&client, &database, &table),
        Commands::Query { database, sql, json, csv } => {
            commands::query::run(&client, &database, &sql, json, csv)
        }
    }
}
