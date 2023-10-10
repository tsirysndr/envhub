use anyhow::Error;
use envhub_types::configuration::File;
use owo_colors::OwoColorize;

use crate::helpers::{read_envhub_file, write_envhub_file};

pub fn add(path: &str, source: Option<&str>, content: Option<&str>) -> Result<(), Error> {
    let mut config = read_envhub_file(".")?;
    let mut files = config.files.unwrap_or_default();
    if files.contains_key(path) {
        println!("File {} already in envhub file", path.cyan());
        return Ok(());
    }
    let file = File {
        source: source.map(|x| x.to_string()),
        content: content.map(|x| x.to_string()),
    };
    files.insert(path.to_string(), file);
    config.files = Some(files);
    write_envhub_file(".", &config)?;
    println!("File {} added to envhub file", path.cyan());
    Ok(())
}

pub fn list() -> Result<(), Error> {
    let config = read_envhub_file(".")?;
    if config.files.is_none() {
        println!("No files found in envhub file");
        return Ok(());
    }
    for (path, file) in config.files.unwrap_or_default() {
        println!("* {}", path.cyan());
        if file.source.is_some() {
            println!("  source: {}", file.source.unwrap().cyan());
        }
        if file.content.is_some() {
            println!("  content: \n{}\n", file.content.unwrap().cyan());
        }
    }
    Ok(())
}

pub fn remove(key: &str) -> Result<(), Error> {
    let mut config = read_envhub_file(".")?;
    let mut files = config.files.unwrap_or_default();
    if !files.contains_key(key) {
        println!("{} File {} not in envhub file", "[✗]".red(), key.cyan());
        return Ok(());
    }
    files.remove(key);
    config.files = Some(files);
    write_envhub_file(".", &config)?;
    println!(
        "{} File {} removed from envhub file",
        "[✓]".bright_green(),
        key.cyan()
    );
    Ok(())
}
