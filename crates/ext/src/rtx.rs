use std::{
    env, fs,
    process::{Command, Stdio},
};

use anyhow::Error;
use envhub_types::configuration::Configuration;

use crate::Extension;

pub struct Rtx {}

impl Rtx {
    pub fn new() -> Self {
        Self {}
    }

    pub fn install(&self, package: &str) -> Result<(), Error> {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("rtx install {}", package))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }
}

impl Extension for Rtx {
    fn load(&self, config: &Configuration) -> Result<(), Error> {
        self.setup()?;
        match config.rtx {
            Some(ref rtx) => {
                for package in &rtx.packages {
                    self.install(package)?;
                }
            }
            None => {}
        }
        Ok(())
    }

    fn setup(&self) -> Result<(), Error> {
        env::set_var(
            "PATH",
            format!(
                "{}/.local/share/rtx/bin:{}",
                env::var("HOME")?,
                env::var("PATH")?
            ),
        );
        env::set_var(
            "PATH",
            format!(
                "{}/.local/share/rtx/shims:{}",
                env::var("HOME")?,
                env::var("PATH")?
            ),
        );
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("type rtx > /dev/null || curl https://rtx.pub/install.sh | sh")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;

        fs::create_dir_all(format!("{}/.local/share/rtx/shims", env::var("HOME")?))?;

        Ok(())
    }
}
