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
    let linux = match std::env::consts::OS {
        "linux" => "linux --extra-conf 'sandbox = false' --init none",
        _ => "",
    };
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!("type nix > /dev/null || curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install {}", linux))
        .spawn()?;
    child.wait()?;

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!("type nix > /dev/null || curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install {} --no-confirm", linux))
        .spawn()?;
    child.wait()?;

    Ok(())
}
