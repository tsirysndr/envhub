use std::{env, process::Command};

use anyhow::Error;

pub fn install() -> Result<(), Error> {
    env::set_var(
        "PATH",
        format!(
            "{}:{}",
            env::var("PATH")?,
            "/nix/var/nix/profiles/default/bin"
        ),
    );
    let mut child = Command::new("sh")
        .arg("-c")
        .arg("type nix > /dev/null || curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install")
        .spawn()?;
    child.wait()?;

    let mut child = Command::new("sh")
        .arg("-c")
        .arg("type nix > /dev/null || curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install --no-confirm")
        .spawn()?;
    child.wait()?;

    Ok(())
}
