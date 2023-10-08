use std::{env, process::Command};

use anyhow::Error;

use crate::PackageManager;

pub struct Nix {}

impl Nix {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageManager for Nix {
    fn install(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn uninstall(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn setup(&self) -> Result<(), Error> {
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
        Ok(())
    }
}
