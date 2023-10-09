use anyhow::Error;
use envhub_hm::reset::reset_env;

pub fn unuse_environment() -> Result<(), Error> {
    reset_env()?;
    Ok(())
}
