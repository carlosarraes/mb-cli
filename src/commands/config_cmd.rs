use anyhow::Result;
use std::io::{self, Write};

use crate::config::{self, Config, MetabaseConfig};

pub fn run() -> Result<()> {
    let url = prompt("Metabase URL (e.g. https://metabase.example.com): ")?;

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
            println!("\nTo get your session token:");
            println!("  1. Log into Metabase in your browser");
            println!("  2. DevTools -> Application -> Cookies");
            println!("  3. Copy the 'metabase.SESSION' value");
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
    config::save(&config)?;

    let path = config::config_path()?;
    println!("Config saved to {}", path.display());
    Ok(())
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
