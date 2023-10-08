use std::process::{Command, Stdio};

use anyhow::Error;

use crate::nix;

pub fn switch_env(dir: Option<&str>) -> Result<(), Error> {
    nix::install()?;
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "nix run home-manager/master -- switch --flake {}",
            dir.unwrap_or("~/.envhub/home-manager")
        ))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    child.wait()?;

    Ok(())
}
