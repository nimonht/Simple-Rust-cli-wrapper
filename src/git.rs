use anyhow::{Context, Result};
use std::process::Command;

pub fn run_git(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("Failed to run: git {}", args.join(" ")))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(anyhow::anyhow!("git {} failed: {}", args.join(" "), stderr))
    }
}

pub fn run_gh(args: &[&str]) -> Result<String> {
    let output = Command::new("gh")
        .args(args)
        .output()
        .with_context(|| format!("Failed to run: gh {}", args.join(" ")))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(anyhow::anyhow!("gh {} failed: {}", args.join(" "), stderr))
    }
}

/// Detect whether the default branch is "master" or "main".
pub fn detect_default_branch() -> Result<String> {
    if run_git(&["rev-parse", "--verify", "master"]).is_ok() {
        Ok("master".to_string())
    } else if run_git(&["rev-parse", "--verify", "main"]).is_ok() {
        Ok("main".to_string())
    } else {
        Err(anyhow::anyhow!(
            "Could not detect default branch (tried 'master' and 'main')"
        ))
    }
}
