use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::Error;
use envhub_types::configuration::ConfigFormat;
use owo_colors::OwoColorize;

use crate::config::{generate_config, generate_default_config};

pub fn execute_init(
    cfg_format: ConfigFormat,
    packages: Vec<String>,
    envs: Vec<String>,
    package_manager: &str,
    use_stow: bool,
) -> Result<(), Error> {
    fs::create_dir_all("dotfiles")?;

    let mut file = File::create("dotfiles/.gitignore")?;
    file.write_all(b"secrets\n")?;

    if packages.is_empty() && envs.is_empty() {
        let mut file = File::create("dotfiles/.zshrc")?;
        file.write_all(b"# This is an example of .zshrc file\n")?;
        generate_default_config(&cfg_format)?;
        println!(
            "{} Successfully initialized a new environment ✨",
            "[✓]".bright_green()
        );
        println!(
            "Run {} to apply the changes to your environment",
            "`envhub use`".bright_green()
        );
        return Ok(());
    }
    generate_config(&cfg_format, packages, envs, package_manager, use_stow)?;
    println!(
        "{} Successfully initialized a new environment ✨",
        "[✓]".bright_green()
    );
    println!(
        "Run {} to apply the changes to your environment",
        "`envhub use`".bright_green()
    );
    Ok(())
}
