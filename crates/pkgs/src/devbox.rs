use anyhow::Error;

use crate::PackageManager;

pub struct Devbox {}

impl Devbox {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageManager for Devbox {
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
