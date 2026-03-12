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

/// Detect the repository's default branch.
///
/// First checks the remote HEAD via `origin/HEAD` (most reliable), which
/// may return any branch name (e.g. "main", "master", "trunk", "develop").
/// Falls back to checking local branches with "main" preferred over "master".
pub fn detect_default_branch() -> Result<String> {
    // First try to detect from the remote HEAD (most reliable)
    if let Ok(remote_head) = run_git(&["symbolic-ref", "refs/remotes/origin/HEAD"]) {
        // Output looks like "refs/remotes/origin/main"
        if let Some(branch) = remote_head.strip_prefix("refs/remotes/origin/") {
            return Ok(branch.to_string());
        }
    }

    // Fall back to checking local branches
    if run_git(&["rev-parse", "--verify", "main"]).is_ok() {
        Ok("main".to_string())
    } else if run_git(&["rev-parse", "--verify", "master"]).is_ok() {
        Ok("master".to_string())
    } else {
        Err(anyhow::anyhow!(
            "Could not detect default branch (tried 'main' and 'master')"
        ))
    }
}
