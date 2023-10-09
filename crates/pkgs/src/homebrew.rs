use crate::PackageManager;
use anyhow::Error;
use std::process::{Command, Stdio};

pub struct Homebrew {}

impl Homebrew {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageManager for Homebrew {
    fn install(&self, name: &str) -> Result<(), Error> {
        self.setup()?;
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("brew install {}", name))
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
            .arg(format!("brew uninstall {}", name))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }

    fn setup(&self) -> Result<(), Error> {
        std::env::set_var(
            "PATH",
            format!(
                "{}:{}",
                std::env::var("PATH").unwrap(),
                "/home/linuxbrew/.linuxbrew/bin"
            ),
        );
        let mut child = Command::new("sh")
          .arg("-c")
          .arg(r#"type brew > /dev/null || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#)
          .stdin(Stdio::inherit())
          .stdout(Stdio::inherit())
          .stderr(Stdio::inherit())
          .spawn()?;
        child.wait()?;
        Ok(())
    }
}
