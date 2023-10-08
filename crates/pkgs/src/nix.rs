use anyhow::Error;

use crate::PackageManager;

pub struct Nix {}

impl Nix {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageManager for Nix {
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
