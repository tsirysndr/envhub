use std::process::{Command, Stdio};

use crate::PackageManager;
use anyhow::Error;

pub struct Pkgx {}

impl Pkgx {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageManager for Pkgx {
    fn install(&self, name: &str) -> Result<(), Error> {
        self.setup()?;
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("pkgx install {}", name))
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
            .arg(format!("rm ~/.local/bin/{}", name))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }

    fn setup(&self) -> Result<(), Error> {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("type pkgx > /dev/null || curl -fsS https://pkgx.sh | sh")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }
}
