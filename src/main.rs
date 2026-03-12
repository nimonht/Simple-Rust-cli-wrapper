use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process::Command;

/// A simple Git workflow automation CLI
#[derive(Parser)]
#[command(name = "git-workflow", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync master and start a new feature branch
    Start {
        /// Name of the new branch to create
        branch_name: String,
    },
    /// Stage all changes, commit, push, and open a Pull Request.
    /// Note: stages ALL tracked and untracked files via 'git add .'.
    /// PR body is auto-filled from commit messages via 'gh pr create --fill'.
    Finish {
        /// Title for the commit and the Pull Request
        pr_title: String,
    },
}

fn run_git(args: &[&str]) -> Result<String> {
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

fn run_gh(args: &[&str]) -> Result<String> {
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

fn cmd_start(branch_name: &str) -> Result<()> {
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

fn cmd_finish(pr_title: &str) -> Result<()> {
    println!("{}", "Staging all changes (git add .)...".cyan());
    println!(
        "{}",
        "  Note: this stages ALL files in the working tree. Ensure .gitignore is configured correctly.".yellow()
    );
    run_git(&["add", "."]).context("Failed to stage changes")?;

    println!(
        "{} {}",
        "Committing with message:".cyan(),
        pr_title.yellow()
    );
    run_git(&["commit", "-m", pr_title]).context("Failed to commit changes")?;

    // Determine current branch name
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

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Start { branch_name } => cmd_start(branch_name),
        Commands::Finish { pr_title } => cmd_finish(pr_title),
    };

    if let Err(err) = result {
        eprintln!("{} {err:#}", "[ERROR]".red().bold());
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli_parse_start() {
        let cli = Cli::try_parse_from(["git-workflow", "start", "feature/my-feature"]).unwrap();
        match cli.command {
            Commands::Start { branch_name } => assert_eq!(branch_name, "feature/my-feature"),
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn verify_cli_parse_finish() {
        let cli = Cli::try_parse_from(["git-workflow", "finish", "Add awesome feature"]).unwrap();
        match cli.command {
            Commands::Finish { pr_title } => assert_eq!(pr_title, "Add awesome feature"),
            _ => panic!("Expected Finish command"),
        }
    }

    #[test]
    fn verify_cli_unknown_subcommand_fails() {
        let result = Cli::try_parse_from(["git-workflow", "unknown"]);
        assert!(result.is_err());
    }

    #[test]
    fn verify_cli_start_missing_arg_fails() {
        let result = Cli::try_parse_from(["git-workflow", "start"]);
        assert!(result.is_err());
    }

    #[test]
    fn verify_cli_finish_missing_arg_fails() {
        let result = Cli::try_parse_from(["git-workflow", "finish"]);
        assert!(result.is_err());
    }
}
