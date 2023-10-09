use std::process::Command;

use anyhow::Error;

use crate::install::install_stow;

pub fn stow(dir: &str, target: &str, package: &str) -> Result<(), Error> {
    install_stow()?;
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!("stow -t {} -d {} {}", target, dir, package))
        .spawn()?;
    child.wait()?;
    Ok(())
}
