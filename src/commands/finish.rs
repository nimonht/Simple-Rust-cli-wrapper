use anyhow::{Context, Result};
use colored::Colorize;

use crate::git::{run_gh, run_git};

pub fn cmd_finish(pr_title: &str) -> Result<()> {
    println!("{}", "Staging all changes (git add .)...".cyan());
    println!(
        "{}",
        "  Note: this stages ALL files in the working tree. Ensure .gitignore is configured correctly."
            .yellow()
    );
    run_git(&["add", "."]).context("Failed to stage changes")?;

    println!(
        "{} {}",
        "Committing with message:".cyan(),
        pr_title.yellow()
    );
    run_git(&["commit", "-m", pr_title]).context("Failed to commit changes")?;

    let branch = run_git(&["rev-parse", "--abbrev-ref", "HEAD"])
        .context("Failed to determine current branch")?;

    if branch.is_empty() || branch == "HEAD" {
        return Err(anyhow::anyhow!(
            "Cannot determine a valid current branch (got '{}'). \
             You may be in a detached HEAD state. \
             Please checkout a branch and rerun this command.",
            branch
        ));
    }

    println!("{} {}", "Pushing branch:".cyan(), branch.yellow());
    run_git(&["push", "--set-upstream", "origin", &branch])
        .with_context(|| format!("Failed to push branch '{branch}'"))?;

    println!("{}", "Opening Pull Request...".cyan());
    let pr_url = run_gh(&["pr", "create", "--title", pr_title, "--fill"])
        .context("Failed to open Pull Request (is 'gh' installed and authenticated?)")?;

    println!(
        "{} {}",
        "[OK] Pull Request created:".green().bold(),
        pr_url.yellow().bold()
    );
    Ok(())
}
