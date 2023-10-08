use std::{
    env,
    fs::{self, create_dir_all},
    process::{Command, Stdio},
};

use anyhow::Error;
use clap::ArgMatches;
use envhub_types::configuration::Configuration;

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
        .arg("git add .")
        .current_dir(dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    child.wait()?;
    Ok(())
}

pub fn git_pull(dir: &str) -> Result<(), Error> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg("git pull origin $(git branch --show-current)")
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
            match fs::metadata(format!("{}/.envhub/github/{}/home.nix", home, name)) {
                Ok(_) => {}
                Err(_) => {
                    copy_home_nix(&format!("{}/.envhub/github/{}", home, name))?;
                }
            };
            git_add(&format!("{}/.envhub/github/{}", home, name))?;
            git_pull(&format!("{}/.envhub/github/{}", home, name))?;
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
    let mut path = format!("{}/envhub.toml", dir);

    if !fs::metadata(&path).is_ok() {
        path = format!("{}/envhub.hcl", dir);
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
