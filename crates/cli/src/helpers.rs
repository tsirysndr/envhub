use std::{
    env,
    fs::{self, create_dir_all},
    process::{Command, Stdio},
};

use anyhow::Error;

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
