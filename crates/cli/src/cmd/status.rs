use std::fs;

use anyhow::Error;
use owo_colors::OwoColorize;

pub fn status() -> Result<(), Error> {
    if fs::metadata(format!("{}/.envhub/current", env!("HOME"))).is_err() {
        println!("No environment is currently in use");
        return Ok(());
    }
    let content = fs::read_to_string(format!("{}/.envhub/current", env!("HOME")))?;
    let lines = content.lines().collect::<Vec<&str>>();
    if lines.len() != 2 {
        panic!("Invalid .envhub/current file");
    }
    let home_manager_dir = lines[0];
    let symlink_manager = lines[1];
    println!(
        "{} Environment: {}",
        "[✓]".bright_green(),
        home_manager_dir.bright_green()
    );
    println!(
        "{} Symlink manager: {}",
        "[✓]".bright_green(),
        symlink_manager.bright_green()
    );
    Ok(())
}
