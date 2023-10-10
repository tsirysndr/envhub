use anyhow::Error;
use envhub_types::configuration::Configuration;

pub mod rtx;

pub trait Extension {
    fn load(&self, config: &Configuration) -> Result<(), Error>;
    fn setup(&self) -> Result<(), Error>;
}
