use anyhow::Error;
use owo_colors::OwoColorize;

use crate::helpers::{read_envhub_file, write_envhub_file};

pub fn add(key: &str, value: &str) -> Result<(), Error> {
    let mut config = read_envhub_file(".")?;
    let mut envs = config.envs.unwrap_or_default();
    if envs.contains_key(key) {
        println!("Environment variable {} already in envhub file", key.cyan());
        return Ok(());
    }
    envs.insert(key.to_string(), value.to_string());
    config.envs = Some(envs);
    write_envhub_file(".", &config)?;
    println!("Environment variable {} added to envhub file", key.cyan());
    Ok(())
}

pub fn list() -> Result<(), Error> {
    let config = read_envhub_file(".")?;

    if config.envs.is_none() {
        println!("No environment variables found in envhub file");
        return Ok(());
    }

    println!("Environment variables found in envhub file:\n");
    for (key, value) in config.envs.unwrap_or_default() {
        println!("* {}={}", key.cyan(), value.cyan());
    }
    Ok(())
}

pub fn remove(key: &str) -> Result<(), Error> {
    let mut config = read_envhub_file(".")?;
    let mut envs = config.envs.unwrap_or_default();
    if !envs.contains_key(key) {
        println!(
            "{} Environment variable {} not in envhub file",
            "[✗]".red(),
            key.cyan()
        );
        return Ok(());
    }
    envs.remove(key);
    config.envs = Some(envs);
    write_envhub_file(".", &config)?;
    println!(
        "{} Environement variable {} removed from envhub file",
        "[✓]".bright_green(),
        key.cyan()
    );
    Ok(())
}
