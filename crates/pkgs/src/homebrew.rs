use anyhow::Error;

use crate::PackageManager;

pub struct Homebrew {}

impl Homebrew {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageManager for Homebrew {
    fn install(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn uninstall(&self, _name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn setup(&self) -> Result<(), Error> {
        Ok(())
    }
}
