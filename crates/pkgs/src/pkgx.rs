use std::{
    env, fs,
    process::{Command, Stdio},
};

use crate::PackageManager;
use anyhow::Error;

pub struct Pkgx {}

impl Pkgx {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write_command(&self, name: &str) -> Result<(), Error> {
        let org = name.split("/").next().unwrap();
        let package = name.split("/").last().unwrap();

        let mut command = "#!/bin/sh\n".to_string();
        command.push_str(&format!("exec pkgx +{} -- {} \"$@\"", org, package));
        fs::write(
            format!("{}/.local/bin/{}", env::var("HOME")?, package),
            command,
        )?;

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("chmod +x ~/.local/bin/{}", package))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }

    pub fn setup_path(&self, rc_file: &str) -> Result<(), Error> {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "type pkgx > /dev/null || echo \'PATH=$HOME/.local/bin:$PATH\' >> ~/{}",
                rc_file
            ))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }
}

impl PackageManager for Pkgx {
    fn install(&self, name: &str) -> Result<(), Error> {
        self.setup()?;
        if name.split("/").count() == 2 {
            return self.write_command(name);
        }

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "[ ! -f ~/.local/bin/{} ] && pkgx install {}",
                name, name
            ))
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
        self.setup_path(".bashrc")?;
        self.setup_path(".zshrc")?;
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
