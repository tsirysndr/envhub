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
        let mut child = Command::new("cp")
            .arg("-r")
            .arg(local_dir.to_str().unwrap())
            .arg(dest)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }
}

impl Provider for Local {
    fn load(&self, name: &str) -> Result<(), Error> {
        self.copy(name)?;
        Ok(())
    }
}
