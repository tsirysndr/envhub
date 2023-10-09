use std::fs;

use anyhow::Error;
use envhub_hm::switch::switch_env;
use envhub_providers::{github::Github, local::Local, s3::S3, Provider};
use envhub_stow::stow::stow;

use crate::helpers::{copy_home_nix, get_home_manager_dir, install_packages, read_envhub_file};

pub fn use_environment(name: &str) -> Result<(), Error> {
    let scheme = match name.split(":").collect::<Vec<&str>>().len() > 1 {
        false => "local",
        true => name.split(":").collect::<Vec<&str>>()[0],
    };
    let source: Box<dyn Provider> = match scheme {
        "github" => Box::new(Github::new()),
        "local" => Box::new(Local::new()),
        "s3" => Box::new(S3::new()),
        _ => panic!("Unknown scheme: {}", scheme),
    };
    let name = match name.split(":").collect::<Vec<&str>>().len() > 1 {
        false => name,
        true => name.split(":").collect::<Vec<&str>>()[1],
    };
    source.load(name)?;

    let home_manager_dir = get_home_manager_dir(scheme, name)?;

    copy_home_nix(&home_manager_dir)?;

    let config = read_envhub_file(&home_manager_dir)?;

    if config.package_manager.is_some() && config.package_manager != Some("nix".into()) {
        install_packages(&config)?;
    }

    switch_env(Some(&home_manager_dir), &config)?;

    if config.symlink_manager == Some("stow".into()) {
        let target = std::env::var("HOME")?;
        let package = "dotfiles";
        stow(&home_manager_dir, &target, &package)?;
    }

    fs::write(
        format!("{}/.envhub/current", std::env::var("HOME")?),
        format!(
            "{}\n{}",
            &home_manager_dir,
            &config.symlink_manager.unwrap_or("home-manager".into())
        ),
    )?;

    Ok(())
}
