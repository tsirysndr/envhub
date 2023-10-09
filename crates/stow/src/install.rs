use std::process::Command;

use anyhow::Error;
use envhub_pkgs::{pkgx::Pkgx, PackageManager};

pub fn install_stow() -> Result<(), Error> {
    let pkgx: Box<dyn PackageManager> = Box::new(Pkgx::new());
    pkgx.setup()?;

    std::env::set_var(
        "PATH",
        format!(
            "{}/{}:{}",
            std::env::var("HOME").unwrap(),
            "/.local/bin",
            std::env::var("PATH").unwrap(),
        ),
    );

    let mut child = Command::new("sh")
        .arg("-c")
        .arg("type stow > /dev/null || pkgx install stow")
        .spawn()?;
    child.wait()?;
    Ok(())
}
