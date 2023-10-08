use crate::PackageManager;
use anyhow::Error;

pub struct Pkgx {}

impl Pkgx {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageManager for Pkgx {
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
