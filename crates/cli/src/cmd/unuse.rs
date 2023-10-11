use std::fs;

use anyhow::Error;
use envhub_hm::reset::reset_env;
use envhub_stow::unstow::unstow;
use owo_colors::OwoColorize;

pub fn unuse_environment() -> Result<(), Error> {
    if fs::metadata(format!("{}/.envhub/current", std::env::var("HOME")?)).is_err() {
        println!(
            "{} No environment is currently in use",
            "[✓]".bright_green()
        );
        return Ok(());
    }
    reset_env()?;
    let content = fs::read_to_string(format!("{}/.envhub/current", std::env::var("HOME")?))?;
    let lines = content.lines().collect::<Vec<&str>>();
    if lines.len() != 3 {
        panic!("Invalid .envhub/current file");
    }
    let home_manager_dir = lines[0];
    let symlink_manager = lines[1];

    if symlink_manager == "stow" {
        let target = std::env::var("HOME")?;
        let package = "dotfiles";
        unstow(&home_manager_dir, &target, &package)?;
    }
    fs::remove_file(format!("{}/.envhub/current", std::env::var("HOME")?))?;
    println!("{} Successfully un-used environment", "[✓]".bright_green());
    Ok(())
}
