use anyhow::Error;
use envhub_hm::switch::switch_env;
use envhub_providers::{github::Github, local::Local, s3::S3, Provider};

use crate::helpers::{copy_home_nix, get_home_manager_dir};

pub fn use_environment(name: &str) -> Result<(), Error> {
    let scheme = match name.split(":").collect::<Vec<&str>>().len() > 1 {
        false => "local",
        true => name.split(":").collect::<Vec<&str>>()[0],
    };
    let source: Box<dyn Provider> = match scheme {
        "github" => Box::new(Github::new()),
        "local" => Box::new(Local::new()),
        "s3" => Box::new(S3::new()),
        _ => panic!("Unknown scheme: {}", scheme),
    };
    let name = match name.split(":").collect::<Vec<&str>>().len() > 1 {
        false => name,
        true => name.split(":").collect::<Vec<&str>>()[1],
    };
    source.load(name)?;

    let home_manager_dir = get_home_manager_dir(scheme, name)?;

    copy_home_nix(&home_manager_dir)?;

    switch_env(Some(&home_manager_dir))?;
    Ok(())
}
