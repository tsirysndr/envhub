use std::{
    fs,
    process::{Command, Stdio},
};

use anyhow::Error;

use crate::Provider;

pub struct Local {}

impl Local {
    pub fn new() -> Self {
        Self {}
    }

    pub fn copy(&self, name: &str) -> Result<(), Error> {
        let home = dirs::home_dir().unwrap();
        let home = home.to_str().unwrap();
        let local_dir = fs::canonicalize(name)?;
        let dest = format!("{}/.envhub/local", home);
        let name = local_dir.file_name().unwrap().to_str().unwrap();
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "[ -d {}/{} ] && rm -rf {}/{}",
                dest, name, dest, name
            ))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("cp -r {} {}", local_dir.to_str().unwrap(), dest))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }
}

impl Provider for Local {
    fn name(&self) -> &str {
        "local"
    }
    fn load(&self, name: &str) -> Result<(), Error> {
        self.copy(name)?;
        Ok(())
    }
}
