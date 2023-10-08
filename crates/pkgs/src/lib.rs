use anyhow::Error;

pub mod devbox;
pub mod homebrew;
pub mod nix;
pub mod pkgx;

pub trait PackageManager {
    fn install(&self, name: &str) -> Result<(), Error>;
    fn uninstall(&self, name: &str) -> Result<(), Error>;
    fn setup(&self) -> Result<(), Error>;
}
