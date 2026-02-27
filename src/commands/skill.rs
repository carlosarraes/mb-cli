use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use std::fs;
use std::os::unix;
use std::path::PathBuf;
use std::time::Duration;

const REPO_BASE_URL: &str = "https://raw.githubusercontent.com/carlosarraes/mb-cli/main/";

const SKILL_FILES: &[&str] = &[
    "skills/mb/SKILL.md",
    "skills/mb/references/flags.md",
];

struct Agent {
    name: &'static str,
    dir: &'static str,
}

const AGENTS: &[Agent] = &[
    Agent { name: "Claude", dir: ".claude/skills" },
    Agent { name: "Cursor", dir: ".cursor/skills" },
    Agent { name: "Codex", dir: ".codex/skills" },
];

fn home_dir() -> Result<PathBuf> {
    dirs::home_dir().context("could not determine home directory")
}

fn skill_dir() -> Result<PathBuf> {
    let dir = dirs::config_dir()
        .context("could not determine config directory")?
        .join("mb/skills");
    Ok(dir)
}

fn is_installed() -> bool {
    skill_dir()
        .map(|d| d.join(".version").exists())
        .unwrap_or(false)
}

fn installed_version() -> Result<String> {
    let path = skill_dir()?.join(".version");
    fs::read_to_string(&path)
        .map(|s| s.trim().to_string())
        .context("could not read installed version")
}

fn fetch_remote_version(client: &Client) -> Result<String> {
    let url = format!("{REPO_BASE_URL}skills/version");
    let resp = client.get(&url).send().context("failed to fetch version")?;
    if !resp.status().is_success() {
        bail!("failed to fetch version: {}", resp.status());
    }
    Ok(resp.text()?.trim().to_string())
}

fn fetch_and_store(client: &Client) -> Result<String> {
    let dir = skill_dir()?;
    let version = fetch_remote_version(client)?;

    for file in SKILL_FILES {
        let url = format!("{REPO_BASE_URL}{file}");
        let resp = client.get(&url).send()
            .with_context(|| format!("failed to fetch {file}"))?;
        if !resp.status().is_success() {
            bail!("failed to fetch {file}: {}", resp.status());
        }
        let body = resp.bytes()?;

        let rel_path = file.strip_prefix("skills/").unwrap_or(file);
        let dest = dir.join(rel_path);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&dest, &body)?;
    }

    fs::write(dir.join(".version"), format!("{version}\n"))?;
    Ok(version)
}

fn detect_agents() -> Result<Vec<&'static Agent>> {
    let home = home_dir()?;
    Ok(AGENTS.iter()
        .filter(|a| home.join(a.dir).is_dir())
        .collect())
}

fn create_symlinks(force: bool) -> Result<Vec<&'static str>> {
    let target = skill_dir()?.join("mb");
    let home = home_dir()?;
    let agents = detect_agents()?;
    let mut linked = Vec::new();

    for agent in agents {
        let link_path = home.join(agent.dir).join("mb");

        if link_path.exists() || link_path.symlink_metadata().is_ok() {
            let meta = link_path.symlink_metadata()?;
            if meta.file_type().is_symlink() {
                fs::remove_file(&link_path)?;
            } else if force {
                fs::remove_dir_all(&link_path)?;
            } else {
                eprintln!("Warning: {} exists and is not a symlink (use --force)", link_path.display());
                continue;
            }
        }

        unix::fs::symlink(&target, &link_path)
            .with_context(|| format!("failed to symlink to {}", link_path.display()))?;
        linked.push(agent.name);
    }

    Ok(linked)
}

fn remove_symlinks() -> Result<Vec<&'static str>> {
    let home = home_dir()?;
    let mut removed = Vec::new();

    for agent in AGENTS {
        let link_path = home.join(agent.dir).join("mb");
        if link_path.symlink_metadata().is_ok() {
            let meta = link_path.symlink_metadata()?;
            if meta.file_type().is_symlink() {
                fs::remove_file(&link_path)?;
                removed.push(agent.name);
            }
        }
    }

    Ok(removed)
}

fn symlink_status() -> Result<Vec<(&'static str, &'static str)>> {
    let home = home_dir()?;
    let mut results = Vec::new();

    for agent in AGENTS {
        let agent_dir = home.join(agent.dir);
        let link_path = agent_dir.join("mb");

        if !agent_dir.is_dir() {
            results.push((agent.name, "not installed"));
            continue;
        }

        match link_path.symlink_metadata() {
            Err(_) => results.push((agent.name, "not linked")),
            Ok(meta) if !meta.file_type().is_symlink() => {
                results.push((agent.name, "directory (not managed by mb)"))
            }
            Ok(_) if link_path.exists() => results.push((agent.name, "linked")),
            Ok(_) => results.push((agent.name, "broken symlink")),
        }
    }

    Ok(results)
}

pub fn add(force: bool) -> Result<()> {
    if is_installed() {
        bail!("skill already installed. Run 'mb skill update' to update or 'mb skill remove' first");
    }

    println!("Fetching mb skill from GitHub...");
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let version = fetch_and_store(&client)?;
    let linked = create_symlinks(force)?;

    println!("Installed mb skill v{version}");
    if linked.is_empty() {
        println!("No AI agents detected. Skill stored at {:?}", skill_dir()?.join("mb"));
    } else {
        println!("Linked to: {}", linked.join(", "));
    }
    Ok(())
}

pub fn update() -> Result<()> {
    if !is_installed() {
        bail!("skill not installed. Run 'mb skill add' first");
    }

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let current = installed_version()?;
    let remote = fetch_remote_version(&client)?;

    if remote == current {
        println!("Already up to date (v{current})");
        return Ok(());
    }

    println!("Updating from v{current} to v{remote}...");
    fetch_and_store(&client)?;
    let linked = create_symlinks(false)?;

    println!("Updated to v{remote}");
    if !linked.is_empty() {
        println!("Linked to: {}", linked.join(", "));
    }
    Ok(())
}

pub fn remove() -> Result<()> {
    if !is_installed() {
        bail!("skill is not installed");
    }

    let removed = remove_symlinks()?;
    let dir = skill_dir()?;
    let _ = fs::remove_dir_all(dir.join("mb"));
    let _ = fs::remove_file(dir.join(".version"));

    println!("Removed mb skill");
    if !removed.is_empty() {
        println!("Unlinked from: {}", removed.join(", "));
    }
    Ok(())
}

pub fn status() -> Result<()> {
    if !is_installed() {
        println!("mb skill: not installed");
        println!("Run 'mb skill add' to install");
        return Ok(());
    }

    let current = installed_version()?;
    println!("mb skill v{current}\n");

    println!("Agents:");
    for (name, status) in symlink_status()? {
        println!("  {name:<8} {status}");
    }

    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
    match fetch_remote_version(&client) {
        Ok(remote) if remote != current => {
            println!("\nUpdate available: v{current} -> v{remote}");
            println!("Run 'mb skill update'");
        }
        Ok(_) => println!("\nUp to date"),
        Err(_) => println!("\nCould not check for updates"),
    }

    Ok(())
}
