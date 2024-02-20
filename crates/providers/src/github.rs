use std::{
    fs,
    process::{Command, Stdio},
};

use anyhow::Error;
use envhub_pkgs::{pkgx::Pkgx, PackageManager};

use crate::Provider;

pub struct Github {}

impl Github {
    pub fn new() -> Self {
        Self {}
    }

    pub fn clone(&self, name: &str) -> Result<(), Error> {
        let pkgx: Box<dyn PackageManager> = Box::new(Pkgx::new());
        pkgx.setup()?;
        let home = dirs::home_dir().unwrap();
        let home = home.to_str().unwrap();
        let user = name.split('/').nth(0).unwrap();
        let dest = name.split('/').last().unwrap();
        let branch = dest.split('@').nth(1);
        let dest = dest.split('@').nth(0).unwrap();
        let name = name.split('@').nth(0).unwrap();
        let checkout = match branch {
            Some(branch) => format!("&& cd {} && pkgx git checkout {}", dest, branch),
            None => "".into(),
        };

        if fs::metadata(format!("{}/.envhub/github/{}/{}", home, user, dest)).is_ok() {
            let mut child = Command::new("sh")
                .arg("-c")
                .arg("pkgx git pull origin $(git branch --show-current)")
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
            .arg(format!("pkgx git clone https://github.com/{} {} {}", name, dest, checkout))
            .current_dir(format!("{}/.envhub/github/{}", home, user))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        let status = child.wait()?;
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to clone {}", name));
        }
        Ok(())
    }
}

impl Provider for Github {
    fn name(&self) -> &str {
        "github"
    }
    fn load(&self, repo: &str) -> Result<(), Error> {
        self.clone(repo)?;
        Ok(())
    }
}
