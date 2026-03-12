mod commands;
mod git;
mod tui;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

/// A simple Git workflow automation CLI
#[derive(Parser)]
#[command(name = "git-workflow", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync the default branch and start a new feature branch
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
    /// Dump commits from a branch to patch or diff files.
    /// Useful for kernel development workflows where patches are
    /// submitted via email.
    Dump {
        /// Branch to dump commits from (defaults to current branch)
        #[arg(short, long)]
        branch: Option<String>,

        /// Specific commit SHA to dump (single commit only)
        #[arg(short, long, required_unless_present = "all", conflicts_with = "all")]
        commit: Option<String>,

        /// Dump all commits unique to the branch compared to the default branch
        #[arg(short, long, conflicts_with = "commit")]
        all: bool,

        /// Output format: patch or diff
        #[arg(short, long, default_value = "patch")]
        format: String,

        /// Output directory path
        #[arg(short, long, default_value = ".")]
        output: String,

        /// Send patches via git send-email to this address (requires --format patch)
        #[arg(short, long)]
        email: Option<String>,
    },
    /// Launch the interactive TUI
    Tui,
}

fn main() {
    let cli = Cli::parse();

    let result: Result<()> = match &cli.command {
        Commands::Start { branch_name } => commands::cmd_start(branch_name),
        Commands::Finish { pr_title } => commands::cmd_finish(pr_title),
        Commands::Dump {
            branch,
            commit,
            all,
            format,
            output,
            email,
        } => commands::cmd_dump(
            branch.as_deref(),
            commit.as_deref(),
            *all,
            format,
            output,
            email.as_deref(),
        ),
        Commands::Tui => tui::run_tui(),
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

    #[test]
    fn verify_cli_parse_dump_requires_target() {
        // Must provide --commit or --all
        let result = Cli::try_parse_from(["git-workflow", "dump"]);
        assert!(result.is_err());
    }

    #[test]
    fn verify_cli_parse_dump_with_options() {
        let cli = Cli::try_parse_from([
            "git-workflow",
            "dump",
            "--branch",
            "feature/x",
            "--commit",
            "abc123",
            "--format",
            "diff",
            "--output",
            "/tmp/patches",
        ])
        .unwrap();
        match cli.command {
            Commands::Dump {
                branch,
                commit,
                all,
                format,
                output,
                email,
            } => {
                assert_eq!(branch.unwrap(), "feature/x");
                assert_eq!(commit.unwrap(), "abc123");
                assert!(!all);
                assert_eq!(format, "diff");
                assert_eq!(output, "/tmp/patches");
                assert!(email.is_none());
            }
            _ => panic!("Expected Dump command"),
        }
    }

    #[test]
    fn verify_cli_parse_dump_all_flag() {
        let cli = Cli::try_parse_from(["git-workflow", "dump", "--all"]).unwrap();
        match cli.command {
            Commands::Dump { all, commit, .. } => {
                assert!(all);
                assert!(commit.is_none());
            }
            _ => panic!("Expected Dump command"),
        }
    }

    #[test]
    fn verify_cli_dump_commit_and_all_conflict() {
        let result = Cli::try_parse_from(["git-workflow", "dump", "--commit", "abc123", "--all"]);
        assert!(result.is_err());
    }

    #[test]
    fn verify_cli_parse_tui() {
        let cli = Cli::try_parse_from(["git-workflow", "tui"]).unwrap();
        assert!(matches!(cli.command, Commands::Tui));
    }
}
