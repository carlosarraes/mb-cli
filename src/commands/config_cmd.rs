use anyhow::{Context, Result};
use std::io::{self, Write};

use crate::config::{self, Config, MetabaseConfig};

pub fn run(cookie_only: bool) -> Result<()> {
    if cookie_only {
        return run_cookie_only();
    }

    let existing = config::load().ok();

    let url = prompt_with_default(
        "Metabase URL (e.g. https://metabase.example.com)",
        existing.as_ref().map(|c| c.metabase.url.as_str()),
    )?;

    if !url.starts_with("https://") {
        eprintln!("Warning: using HTTP — credentials will be sent in plaintext");
    }

    println!("\nAuth method:");
    println!("  1) API key (requires admin access)");
    println!("  2) Session token (from browser cookie)");
    let choice = prompt("Choose [1/2]: ")?;

    let (api_key, session_token) = match choice.as_str() {
        "1" => (Some(secret_prompt("API key: ")?), None),
        "2" => {
            print_cookie_instructions();
            (None, Some(secret_prompt("Session token: ")?))
        }
        _ => {
            println!("Invalid choice, defaulting to session token");
            (None, Some(secret_prompt("Session token: ")?))
        }
    };

    let config = Config {
        metabase: MetabaseConfig {
            url: url.trim_end_matches('/').to_string(),
            api_key,
            session_token,
        },
    };
    save_config(&config)
}

fn run_cookie_only() -> Result<()> {
    let existing = config::load().context("no existing config — run `mb config` first")?;

    print_cookie_instructions();
    let token = secret_prompt("Session token: ")?;

    let config = Config {
        metabase: MetabaseConfig {
            url: existing.metabase.url,
            api_key: None,
            session_token: Some(token),
        },
    };
    save_config(&config)
}

fn save_config(config: &Config) -> Result<()> {
    config::save(config)?;
    let path = config::config_path()?;
    println!("Config saved to {}", path.display());
    Ok(())
}

fn print_cookie_instructions() {
    println!("\nTo get your session token:");
    println!("  1. Log into Metabase in your browser");
    println!("  2. DevTools -> Application -> Cookies");
    println!("  3. Copy the 'metabase.SESSION' value");
}

fn prompt_with_default(message: &str, default: Option<&str>) -> Result<String> {
    let input = match default {
        Some(val) => prompt(&format!("{message} [{val}]: "))?,
        None => prompt(&format!("{message}: "))?,
    };
    if input.is_empty() {
        if let Some(val) = default {
            return Ok(val.to_string());
        }
    }
    Ok(input)
}

fn prompt(message: &str) -> Result<String> {
    print!("{message}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn secret_prompt(message: &str) -> Result<String> {
    let input = rpassword::prompt_password(message)?;
    Ok(input.trim().to_string())
}
