use anyhow::Error;

use crate::Provider;

pub struct S3 {}

impl S3 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn download(&self, _dir: &str) -> Result<(), Error> {
        Ok(())
    }
}

impl Provider for S3 {
    fn load(&self, name: &str) -> Result<(), Error> {
        self.download(name)?;
        Ok(())
    }
}
