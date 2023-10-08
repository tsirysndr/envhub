use std::process::{Command, Stdio};

use anyhow::Error;

use crate::nix;

pub fn home_manager_init() -> Result<(), Error> {
    nix::install()?;
    let mut child = Command::new("sh")
        .arg("-c")
        .arg("[ ! -d ~/.envhub/home-manager ] && nix run home-manager/master -- init ~/.envhub/home-manager")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    child.wait()?;

    Ok(())
}
