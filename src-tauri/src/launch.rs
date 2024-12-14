use std::process::Command;

pub fn launch_minecraft(java_path: &str, args: Vec<&str>) -> anyhow::Result<()> {
    Command::new(java_path).args(args).spawn()?.wait()?;
    Ok(())
}