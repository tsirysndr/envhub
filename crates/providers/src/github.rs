use std::{
    fs,
    process::{Command, Stdio},
};

use anyhow::Error;

use crate::Provider;

pub struct Github {}

impl Github {
    pub fn new() -> Self {
        Self {}
    }

    pub fn clone(&self, url: &str, name: &str) -> Result<(), Error> {
        let home = dirs::home_dir().unwrap();
        let home = home.to_str().unwrap();
        let user = name.split('/').nth(0).unwrap();
        let dest = name.split('/').last().unwrap();

        if fs::metadata(format!("{}/.envhub/github/{}", home, user)).is_ok() {
            let mut child = Command::new("sh")
                .arg("-c")
                .arg(format!("git pull origin $(git branch --show-current)"))
                .current_dir(format!("{}/.envhub/github/{}/{}", home, user, dest))
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;
            child.wait()?;
            return Ok(());
        }

        fs::create_dir_all(format!("{}/.envhub/github/{}", home, user))?;

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("git clone {} {}", url, dest))
            .current_dir(format!("{}/.envhub/github/{}", home, user))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        child.wait()?;
        Ok(())
    }
}

impl Provider for Github {
    fn load(&self, repo: &str) -> Result<(), Error> {
        let url = format!("https://github.com/{}", repo);
        self.clone(&url, repo)?;
        Ok(())
    }
}
