use anyhow::Error;

pub fn unstow(dir: &str, target: &str, package: &str) -> Result<(), Error> {
    let mut child = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("stow -t {} -d {} -D {}", target, dir, package))
        .spawn()?;
    child.wait()?;
    Ok(())
}
