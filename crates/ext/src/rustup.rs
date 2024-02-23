use std::{
    env,
    process::{Command, Stdio},
};

use anyhow::Error;
use envhub_types::configuration::Configuration;

use crate::Extension;

pub struct Rustup {}

impl Rustup {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set_default_toolchain(&self, toolchain: &str) -> Result<(), Error> {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "rustup toolchain install {} && rustup default {}",
                toolchain, toolchain
            ))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }

    pub fn install_toolchains(&self, toolchains: Vec<String>) -> Result<(), Error> {
        for toolchain in toolchains {
            let mut child = Command::new("sh")
                .arg("-c")
                .arg(format!("rustup toolchain install {}", toolchain))
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;
            child.wait()?;
        }
        Ok(())
    }

    pub fn install_components(&self, components: Vec<String>) -> Result<(), Error> {
        for component in components {
            let mut child = Command::new("sh")
                .arg("-c")
                .arg(format!("rustup component add {}", component))
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;
            child.wait()?;
        }
        Ok(())
    }

    pub fn add_targets(&self, targets: Vec<String>) -> Result<(), Error> {
        for target in targets {
            let mut child = Command::new("sh")
                .arg("-c")
                .arg(format!("rustup target add {}", target))
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;
            child.wait()?;
        }
        Ok(())
    }
}

impl Extension for Rustup {
    fn load(&self, config: &Configuration) -> Result<(), Error> {
        self.setup()?;
        match config.rustup {
            Some(ref rustup) => {
                if let Some(value) = &rustup.default {
                    self.set_default_toolchain(value)?;
                }
                if let Some(value) = &rustup.toolchains {
                    self.install_toolchains(value.clone())?;
                }
                if let Some(value) = &rustup.components {
                    self.install_components(value.clone())?;
                }
                if let Some(value) = &rustup.targets {
                    self.add_targets(value.clone())?;
                }
            }
            None => {}
        }
        Ok(())
    }

    fn setup(&self) -> Result<(), Error> {
        env::set_var(
            "PATH",
            format!("{}/.cargo/bin:{}", env::var("HOME")?, env::var("PATH")?),
        );
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;

        Ok(())
    }
}
