use std::{
    fs,
    process::{Command, Stdio},
};

use anyhow::Error;
use envhub_types::configuration::Configuration;

use crate::nix;

const HOME_MANAGER: &str = "~/.envhub/home-manager";

pub fn switch_env(dir: Option<&str>, config: &Configuration) -> Result<(), Error> {
    nix::install()?;
    let home_nix = fs::read_to_string(format!("{}/home.nix", dir.unwrap_or(HOME_MANAGER)))?;
    let updated_home_nix = add_packages(&home_nix, config.packages.clone().unwrap_or_default())?;
    let home_nix_file = format!("{}/home.nix", dir.unwrap_or(HOME_MANAGER));
    fs::write(&home_nix_file, updated_home_nix)?;

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "nix run home-manager/master -- switch --flake {}",
            dir.unwrap_or(HOME_MANAGER)
        ))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    child.wait()?;

    Ok(())
}

pub fn add_packages(content: &str, pkgs: Vec<String>) -> Result<String, Error> {
    let mut packages = nix_editor::read::readvalue(content, "home.packages")?;
    let mut entry = String::new();
    for pkg in pkgs {
        let pkg = match pkg.starts_with("pkgs.") {
            true => pkg.to_string(),
            false => format!("pkgs.{}", pkg),
        };
        if packages.contains(&pkg) && !packages.contains(&format!("# {}", pkg)) {
            continue;
        }
        entry.push_str(&format!("\n  {}", pkg));
    }
    entry.push_str("\n]");

    packages = packages.replace("\n]", &entry);

    // replace all \n with \n  to keep the formatting
    packages = packages.replace("\n", "\n  ");

    let output = nix_editor::write::write(content, "home.packages", &packages)?;
    Ok(output)
}
