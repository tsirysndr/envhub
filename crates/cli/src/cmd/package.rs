use anyhow::Error;
use envhub_hm::switch::switch_env;

pub fn add(_package: &str) -> Result<(), Error> {
    switch_env(None)?;
    Ok(())
}

pub fn list() -> Result<(), Error> {
    Ok(())
}

pub fn remove(_package: &str) -> Result<(), Error> {
    switch_env(None)?;
    Ok(())
}
