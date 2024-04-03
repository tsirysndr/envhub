use crate::PackageManager;
use anyhow::Error;
use std::process::{Command, Stdio};

pub struct Homebrew {}

impl Homebrew {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_install_command(&self, name: &str) -> Command {
        let mut command = Command::new("sh");
        command
            .arg("-c")
            .arg(format!("brew install {}", name))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        command
    }
}

impl Default for Homebrew {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageManager for Homebrew {
    fn install(&self, name: &str) -> Result<(), Error> {
        self.setup()?;
        let mut child = self.create_install_command(name).spawn()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_child_command_with_version() {
        let homebrew = Homebrew::new();
        let command = homebrew.create_install_command("neovim@0.9.5");
        assert_eq!(command.get_args().len(), 2);
        assert_eq!(
            command.get_args().last().unwrap().to_string_lossy(),
            "brew install neovim@0.9.5"
        );
    }

    #[test]
    fn test_create_child_command_without_version() {
        let homebrew = Homebrew::new();
        let command = homebrew.create_install_command("neovim");
        assert_eq!(command.get_args().len(), 2);
        assert_eq!(
            command.get_args().last().unwrap().to_string_lossy(),
            "brew install neovim"
        );
    }

    #[test]
    fn test_create_child_command_with_head() {
        let homebrew = Homebrew::new();
        let command = homebrew.create_install_command("neovim --HEAD");
        assert_eq!(command.get_args().len(), 2);
        assert_eq!(
            command.get_args().last().unwrap().to_string_lossy(),
            "brew install neovim --HEAD"
        );
    }
}
