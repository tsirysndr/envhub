use anyhow::Error;

pub mod github;
pub mod local;
pub mod s3;

pub trait Provider {
    fn load(&self, name: &str) -> Result<(), Error>;
}
