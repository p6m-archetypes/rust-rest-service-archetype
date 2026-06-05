use anyhow::Result;
use std::process::Command;

fn main() -> Result<()> {
    let task = std::env::args().nth(1);
    match task.as_deref() {
        Some("install") => install(),
        _ => {
            eprintln!("Usage: cargo xtask <task>");
            eprintln!("Tasks: install");
            std::process::exit(1);
        }
    }
}

fn install() -> Result<()> {
    let status = Command::new("cargo")
        .args(["install", "--path", "crates/{{ prefix_name }}_{{ suffix_name }}_bin"])
        .status()?;
    if !status.success() {
        anyhow::bail!("cargo install failed");
    }
    Ok(())
}
