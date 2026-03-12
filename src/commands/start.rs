use anyhow::{Context, Result};
use colored::Colorize;

use crate::git::run_git;

pub fn cmd_start(branch_name: &str) -> Result<()> {
    println!("{}", "Syncing default branch...".cyan());

    run_git(&["checkout", "master"])
        .or_else(|err| {
            let msg = err.to_string();
            if msg.contains("did not match any file") || msg.contains("unknown revision or path") {
                run_git(&["checkout", "main"])
            } else {
                Err(err)
            }
        })
        .context("Failed to switch to default branch (master/main)")?;

    run_git(&["pull"]).context("Failed to pull latest changes")?;

    println!("{} {}", "Creating new branch:".cyan(), branch_name.yellow());
    run_git(&["checkout", "-b", branch_name])
        .with_context(|| format!("Failed to create branch '{branch_name}'"))?;

    println!(
        "{} {}",
        "[OK] Branch created and ready:".green().bold(),
        branch_name.yellow().bold()
    );
    Ok(())
}
