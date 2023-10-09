use std::process::{Command, Stdio};

use anyhow::Error;

use crate::{nix, HOME_MANAGER};

pub fn reset_env() -> Result<(), Error> {
    nix::install()?;
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "nix run home-manager/master -- switch --flake {}",
            HOME_MANAGER
        ))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    child.wait()?;

    Ok(())
}
