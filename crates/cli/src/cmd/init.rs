use anyhow::Error;
use envhub_types::configuration::ConfigFormat;

use crate::config::{generate_config, generate_default_config};

pub fn execute_init(
    cfg_format: ConfigFormat,
    packages: Vec<String>,
    envs: Vec<String>,
    package_manager: &str,
) -> Result<(), Error> {
    if packages.is_empty() && envs.is_empty() {
        generate_default_config(&cfg_format)?;
        return Ok(());
    }
    generate_config(&cfg_format, packages, envs, package_manager)?;
    Ok(())
}
