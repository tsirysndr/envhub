use std::{
    env,
    fs::{self, create_dir_all},
    process::{Command, Stdio},
};

use anyhow::Error;
use clap::ArgMatches;
use envhub_pkgs::{devbox::Devbox, homebrew::Homebrew, pkgx::Pkgx, PackageManager};
use envhub_types::configuration::Configuration;
use indexmap::IndexMap;
use inquire::{Password, PasswordDisplayMode};

pub fn create_envhub_dirs() -> Result<(), Error> {
    let base_dir = format!("{}/.envhub", env::var("HOME")?);
    let github_dir = format!("{}/github", base_dir);
    let local_dir = format!("{}/local", base_dir);
    let s3_dir = format!("{}/s3", base_dir);

    create_dir_all(&base_dir)?;
    create_dir_all(&github_dir)?;
    create_dir_all(&local_dir)?;
    create_dir_all(&s3_dir)?;
    envhub_hm::init::home_manager_init()?;

    Ok(())
}

pub fn copy_home_nix(dest: &str) -> Result<(), Error> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!("cp ~/.envhub/home-manager/* {}", dest))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    child.wait()?;
    Ok(())
}

pub fn git_add(dir: &str) -> Result<(), Error> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg("pkgx git add .")
        .current_dir(dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    child.wait()?;
    Ok(())
}

pub fn get_home_manager_dir(scheme: &str, name: &str) -> Result<String, Error> {
    let home = std::env::var("HOME")?;
    let path = match scheme {
        "github" => {
            let name = name.split("@").nth(0).unwrap();
            match fs::metadata(format!("{}/.envhub/github/{}/home.nix", home, name)) {
                Ok(_) => {}
                Err(_) => {
                    copy_home_nix(&format!("{}/.envhub/github/{}", home, name))?;
                }
            };
            git_add(&format!("{}/.envhub/github/{}", home, name))?;
            format!("{}/.envhub/github/{}", home, name)
        }
        "local" => {
            let local_dir = fs::canonicalize(name)?;
            let dir = local_dir.to_str().unwrap().split("/").last().unwrap();
            format!("{}/.envhub/local/{}", home, dir)
        }
        "s3" => format!("{}/.envhub/s3/{}", home, name),
        _ => panic!("Unknown scheme: {}", scheme),
    };
    Ok(path)
}

pub fn parse_default_package_manager(args: &ArgMatches) -> &str {
    if args.is_present("nix") {
        return "nix";
    }

    if args.is_present("brew") {
        return "homebrew";
    }

    if args.is_present("pkgx") {
        return "pkgx";
    }

    if args.is_present("devbox") {
        return "devbox";
    }

    return "nix";
}

pub fn read_envhub_file(dir: &str) -> Result<Configuration, Error> {
    let mut path = format!("{}/envhub.hcl", dir);

    if !fs::metadata(&path).is_ok() {
        path = format!("{}/envhub.toml", dir);
    }

    if !fs::metadata(&path).is_ok() {
        panic!("No `envhub.toml` or `envhub.hcl` file found in {}", dir)
    }

    let contents = fs::read_to_string(&path)?;
    let ext = path.split(".").last().unwrap();
    match ext {
        "toml" => {
            let config: Configuration = toml::from_str(&contents)?;
            Ok(config)
        }
        "hcl" => {
            let config: Configuration = hcl::from_str(&contents)?;
            Ok(config)
        }
        _ => panic!("Unknown file extension: {}", ext),
    }
}

pub fn write_envhub_file(dir: &str, config: &Configuration) -> Result<(), Error> {
    let mut path = format!("{}/envhub.hcl", dir);

    if !fs::metadata(&path).is_ok() {
        path = format!("{}/envhub.toml", dir);
    }

    if !fs::metadata(&path).is_ok() {
        panic!("No `envhub.toml` or `envhub.hcl` file found in {}", dir)
    }

    let ext = path.split(".").last().unwrap();
    match ext {
        "toml" => {
            let content = toml::to_string_pretty(&config)?;
            fs::write(&path, content)?;
            Ok(())
        }
        "hcl" => {
            let content = hcl::to_string(&config)?;
            fs::write(&path, content)?;
            Ok(())
        }
        _ => panic!("Unknown file extension: {}", ext),
    }
}

fn sync_packages(config: &Configuration) -> Result<(), Error> {
    let current_state = fs::read_to_string(format!("{}/.envhub/envhub.toml", env::var("HOME")?))?;
    let current_config: Configuration = toml::from_str(&current_state)?;
    let current_packages = current_config.packages.clone().unwrap_or_default();
    let packages = config.packages.clone().unwrap_or_default();
    for package in current_packages {
        if !packages.contains(&package) {
            let pm: Box<dyn PackageManager> =
                match current_config.package_manager.as_ref().unwrap().as_str() {
                    "homebrew" => Box::new(Homebrew::new()),
                    "brew" => Box::new(Homebrew::new()),
                    "pkgx" => Box::new(Pkgx::new()),
                    "devbox" => Box::new(Devbox::new()),
                    _ => panic!("Unknown package manager"),
                };
            pm.uninstall(&package)?;
        }
    }
    Ok(())
}

pub fn install_packages(config: &Configuration) -> Result<(), Error> {
    if config.package_manager.is_some() && config.package_manager != Some("nix".into()) {
        if fs::metadata(format!("{}/.envhub/envhub.toml", env::var("HOME")?)).is_ok() {
            self::sync_packages(config)?;
        }

        let packages = config.packages.clone().unwrap_or_default();
        let pm: Box<dyn PackageManager> = match config.package_manager.as_ref().unwrap().as_str() {
            "homebrew" => Box::new(Homebrew::new()),
            "brew" => Box::new(Homebrew::new()),
            "pkgx" => Box::new(Pkgx::new()),
            "devbox" => Box::new(Devbox::new()),
            _ => panic!("Unknown package manager"),
        };

        for package in packages {
            pm.install(&package)?;
        }
    }

    if let Some(brew) = config.homebrew.clone() {
        let pm: Box<dyn PackageManager> = Box::new(Homebrew::new());
        for package in brew.packages {
            pm.install(&package)?;
        }
    }

    if let Some(pkgx) = config.pkgx.clone() {
        let pm: Box<dyn PackageManager> = Box::new(Pkgx::new());
        for package in pkgx.packages {
            pm.install(&package)?;
        }
    }

    if let Some(devbox) = config.devbox.clone() {
        let pm: Box<dyn PackageManager> = Box::new(Devbox::new());
        for package in devbox.packages {
            pm.install(&package)?;
        }
    }

    Ok(())
}

pub fn save_secrets(config: &Configuration) -> Result<(), Error> {
    if let Some(secrets) = config.secrets.clone() {
        let home = env::var("HOME")?;

        let mut secrets_map = IndexMap::new();

        for secret in secrets {
            if let Ok(value) = env::var(&secret) {
                secrets_map.insert(secret, value);
                continue;
            }
            let value = Password::new(&secret)
                .with_display_toggle_enabled()
                .with_display_mode(PasswordDisplayMode::Masked)
                .without_confirmation()
                .prompt()?;
            secrets_map.insert(secret, value);
        }

        let mut content = "#!/bin/sh\n".to_string();
        for (key, value) in secrets_map {
            content = format!("{}export {}={}\n", content, key, value);
        }
        let path = format!("{}/.envhub/secrets", home);
        fs::write(path, content)?;
    }
    Ok(())
}
