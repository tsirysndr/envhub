use anyhow::Error;
use clap::{arg, Arg, Command};
use helpers::{create_envhub_dirs, parse_default_package_manager};

pub mod cmd;
pub mod config;
pub mod helpers;

fn cli() -> Command<'static> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    Command::new("envhub")
        .version(VERSION)
        .about(
            r#"  
           ______           __  __      __  
          / ____/___ _   __/ / / /_  __/ /_ 
         / __/ / __ \ | / / /_/ / / / / __ \
        / /___/ / / / |/ / __  / /_/ / /_/ /
       /_____/_/ /_/|___/_/ /_/\__,_/_.___/ 
                                      
    
    Manage your dotfiles and packages with ease 🚀 ✨"#,
        )
        .author("Tsiry Sandratraina <tsiry.sndr@fluentci.io>")
        .subcommand(
            Command::new("init")
                .arg(arg!(--toml "Generate a default configuration file in toml format"))
                .arg(arg!(--pkgx "Use pkgx as the package manager"))
                .arg(arg!(--nix "Use nix as the package manager"))
                .arg(arg!(--devbox "Use devbox as the package manager"))
                .arg(arg!(--brew "Use homebrew as the package manager"))
                .arg(arg!(--stow "Use GNU Stow as a symlink farm manager. Defaults to 'home-manager'"))
                .arg(
                    Arg::new("pkgs")
                        .alias("packages")
                        .long("pkgs")
                        .short('p')
                        .takes_value(true)
                        .multiple_values(true)
                        .help("Specify packages to add to the environment"),
                )
                .arg(
                    Arg::new("envs")
                        .long("envs")
                        .short('e')
                        .takes_value(true)
                        .multiple_values(true)
                        .help("Specify environment variables to add to the environment"),
                )
                .about("Initialize the environment"),
        )
        .subcommand(
            Command::new("package")
                .about("Manage packages in the environment")
                .alias("pkg")
                .subcommand(
                    Command::new("add")
                        .about("Add a package to the environment")
                        .arg(arg!(<package>)),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a package from the environment")
                        .arg(arg!(<package>)),
                )
                .subcommand(Command::new("list").about("List all packages in the environment")),
        )
        .subcommand(
            Command::new("env")
                .about("Manage environment variables in the environment")
                .subcommand(
                    Command::new("add")
                        .about("Add an environment variable to the environment")
                        .arg(arg!(<key>).required(true).index(1))
                        .arg(arg!(<value>).required(true).index(2)),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove an environment variable from the environment")
                        .arg(arg!(<key>).required(true).index(1)),
                )
                .subcommand(Command::new("list").about("List all environment variables")),
        )
        .subcommand(
            Command::new("file")
                .about("Manage files (dotfiles) in the environment")
                .subcommand(
                    Command::new("add")
                        .about("Add a file to the environment")
                        .arg(arg!(<path>).required(true))
                        .arg(
                            arg!(-s --source [source] "Source file to copy from")
                                .required_unless("content"),
                        )
                        .arg(
                            arg!(-c --content  [content] "Content of the file")
                                .required_unless("source"),
                        ),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a file from the environment")
                        .arg(arg!(<key>).required(true).index(1)),
                )
                .subcommand(Command::new("list").about("List all files")),
        )
        .subcommand(
            Command::new("use")
                .about("Enable an environment, can be a remote repository or a local directory")
                .arg(arg!(<environment>).required(true).index(1)),
        )
        .subcommand(
          Command::new("unuse")
            .about("Restore the previous environment"),
        )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    create_envhub_dirs()?;
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("init", args)) => {
            let envs: Vec<String> = args
                .get_many::<String>("envs")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect();
            let packages: Vec<String> = args
                .get_many::<String>("pkgs")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect();

            match args.is_present("toml") {
                true => cmd::init::execute_init(
                    envhub_types::configuration::ConfigFormat::TOML,
                    packages,
                    envs,
                    parse_default_package_manager(args),
                    args.is_present("stow"),
                )?,
                false => cmd::init::execute_init(
                    envhub_types::configuration::ConfigFormat::HCL,
                    packages,
                    envs,
                    parse_default_package_manager(args),
                    args.is_present("stow"),
                )?,
            }
        }
        Some(("env", args)) => match args.subcommand() {
            Some(("add", args)) => cmd::env::add(
                args.value_of("key").unwrap(),
                args.value_of("value").unwrap(),
            )?,
            Some(("remove", args)) => cmd::env::remove(args.value_of("key").unwrap())?,
            Some(("list", _)) => cmd::env::list()?,
            _ => cli().print_help().unwrap(),
        },
        Some(("package", args)) => match args.subcommand() {
            Some(("add", args)) => cmd::package::add(args.value_of("package").unwrap())?,
            Some(("remove", args)) => cmd::package::remove(args.value_of("package").unwrap())?,
            Some(("list", _)) => cmd::package::list()?,
            _ => cli().print_help().unwrap(),
        },
        Some(("use", args)) => cmd::r#use::use_environment(args.value_of("environment").unwrap())?,
        Some(("unuse", _)) => cmd::unuse::unuse_environment()?,
        _ => cli().print_help().unwrap(),
    }
    Ok(())
}
