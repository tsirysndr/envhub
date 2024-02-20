use anyhow::Error;
use owo_colors::OwoColorize;

use crate::{
    cmd::r#use::use_environment,
    helpers::{read_envhub_file, write_envhub_file},
};

pub fn add(package: &str, apply: bool) -> Result<(), Error> {
    let mut config = read_envhub_file(".")?;
    let mut packages = config.packages.unwrap_or_default();
    if packages.contains(&package.to_string()) {
        println!("Package {} already in envhub file", package.cyan());
        return Ok(());
    }
    packages.push(package.to_string());
    config.packages = Some(packages);
    write_envhub_file(".", &config)?;
    println!("Package {} added to envhub file", package.cyan());
    if apply {
        use_environment(".", false)?;
    }
    Ok(())
}

pub fn list() -> Result<(), Error> {
    let config = read_envhub_file(".")?;
    println!("Packages found in envhub file:\n");
    for package in config.packages.unwrap_or_default() {
        println!("* {}", package.cyan());
    }
    println!("");
    Ok(())
}

pub fn remove(package: &str, apply: bool) -> Result<(), Error> {
    let mut config = read_envhub_file(".")?;
    let mut packages = config.packages.unwrap_or_default();

    if packages.is_empty() {
        println!("No packages found in envhub file");
        return Ok(());
    }

    if !packages.contains(&package.to_string()) {
        println!(
            "{} Package {} not in envhub file",
            "[✗]".red(),
            package.cyan()
        );
        return Ok(());
    }
    packages.retain(|x| x != package);
    config.packages = Some(packages);
    write_envhub_file(".", &config)?;
    println!(
        "{} Package {} removed from envhub file",
        "[✓]".bright_green(),
        package.cyan()
    );
    if apply {
        use_environment(".", false)?;
    }
    Ok(())
}
