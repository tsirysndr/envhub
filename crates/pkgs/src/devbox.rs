use crate::{nix::Nix, PackageManager};
use anyhow::Error;
use std::process::{Command, Stdio};

pub struct Devbox {}

impl Devbox {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageManager for Devbox {
    fn install(&self, name: &str) -> Result<(), Error> {
        self.setup()?;
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("devbox global add {}", name))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), Error> {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("devbox global rm {}", name))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }

    fn setup(&self) -> Result<(), Error> {
        Nix::new().setup()?;
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("type devbox > /dev/null || curl -fsSL https://get.jetpack.io/devbox | bash")
            .spawn()?;
        child.wait()?;
        Ok(())
    }
}
