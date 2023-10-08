use std::{fs::File, io::Write};

use anyhow::Error;
use envhub_types::configuration::{ConfigFormat, Configuration, Packages};

pub fn generate_default_config(cfg_format: &ConfigFormat) -> Result<(), Error> {
    let config = Configuration {
        packages: Some(vec!["hello".into()]),
        envs: Some([("EDITOR".into(), "vim".into())].iter().cloned().collect()),
        files: Some(
            [
                (
                    ".zshrc".into(),
                    envhub_types::configuration::File {
                        source: Some(".zshrc".into()),
                        ..Default::default()
                    },
                ),
                (
                    ".gradle/gradle.properties".into(),
                    envhub_types::configuration::File {
                        content: Some("org.gradle.daemon=true".into()),
                        ..Default::default()
                    },
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        ),
        ..Default::default()
    };

    match cfg_format {
        ConfigFormat::TOML => write_toml(&config)?,
        ConfigFormat::HCL => write_hcl(&config)?,
    }
    Ok(())
}

pub fn generate_config(
    cfg_format: &ConfigFormat,
    packages: Vec<String>,
    envs: Vec<String>,
    package_manager: &str,
) -> Result<(), Error> {
    let mut config = Configuration {
        packages: match package_manager {
            "nix" => Some(packages.clone()),
            _ => None,
        },
        envs: match envs.is_empty() {
            true => None,
            false => Some(
                envs.iter()
                    .map(|s| {
                        let mut split = s.split('=');
                        (
                            split.next().unwrap().to_string(),
                            split.next().unwrap().to_string(),
                        )
                    })
                    .collect(),
            ),
        },
        ..Default::default()
    };

    match package_manager {
        "homebrew" => {
            config.homebrew = Some(Packages { packages });
        }
        "pkgx" => {
            config.pkgx = Some(Packages { packages });
        }
        "devbox" => {
            config.devbox = Some(Packages { packages });
        }
        _ => {}
    };

    match cfg_format {
        ConfigFormat::TOML => write_toml(&config)?,
        ConfigFormat::HCL => write_hcl(&config)?,
    }
    Ok(())
}

fn write_toml(config: &Configuration) -> Result<(), Error> {
    let config = toml::to_string(&config)?;
    let mut file = File::create("envhub.toml").unwrap();
    file.write_all(config.as_bytes()).unwrap();
    Ok(())
}

fn write_hcl(config: &Configuration) -> Result<(), Error> {
    let config = hcl::to_string(&config)?;
    let mut file = File::create("envhub.hcl").unwrap();
    file.write_all(config.as_bytes()).unwrap();
    Ok(())
}
