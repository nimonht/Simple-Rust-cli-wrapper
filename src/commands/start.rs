use anyhow::{Context, Result};
use colored::Colorize;

use crate::git::{detect_default_branch, run_git};

/// Sync the default branch (master or main) and create a new feature branch.
pub fn cmd_start(branch_name: &str) -> Result<()> {
    println!("{}", "Syncing default branch...".cyan());

    let default_branch =
        detect_default_branch().context("Failed to detect default branch (master/main)")?;

    run_git(&["checkout", &default_branch])
        .with_context(|| format!("Failed to switch to default branch '{default_branch}'"))?;

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
