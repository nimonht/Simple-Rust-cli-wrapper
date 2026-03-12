use anyhow::{Context, Result};
use colored::Colorize;

use crate::git::{detect_default_branch, run_git};

/// Dump commits from a branch to patch or diff files.
pub fn cmd_dump(
    branch: Option<&str>,
    commit: Option<&str>,
    _all: bool,
    format: &str,
    output: &str,
    email: Option<&str>,
) -> Result<()> {
    let target_branch = match branch {
        Some(b) => b.to_string(),
        None => run_git(&["rev-parse", "--abbrev-ref", "HEAD"])
            .context("Failed to determine current branch")?,
    };

    println!(
        "{} {}",
        "Dumping commits from branch:".cyan(),
        target_branch.yellow()
    );

    if format != "patch" && format != "diff" {
        return Err(anyhow::anyhow!(
            "Invalid format '{}'. Use 'patch' or 'diff'.",
            format
        ));
    }

    std::fs::create_dir_all(output)
        .with_context(|| format!("Failed to create output directory '{output}'"))?;

    if let Some(sha) = commit {
        dump_specific_commit(sha, format, output)?;
    } else {
        dump_all_commits(&target_branch, format, output)?;
    }

    if let Some(addr) = email {
        send_patches(output, addr)?;
    }

    Ok(())
}

fn dump_specific_commit(sha: &str, format: &str, output: &str) -> Result<()> {
    match format {
        "patch" => {
            println!("{} {}", "Generating patch for commit:".cyan(), sha.yellow());
            let result = run_git(&["format-patch", "-1", sha, "-o", output])
                .with_context(|| format!("Failed to generate patch for commit '{sha}'"))?;
            println!(
                "{} {}",
                "[OK] Patch written:".green().bold(),
                result.yellow()
            );
        }
        "diff" => {
            println!("{} {}", "Generating diff for commit:".cyan(), sha.yellow());
            let diff = run_git(&["show", "--format=", "--patch", sha])
                .with_context(|| format!("Failed to generate diff for commit '{sha}'"))?;
            let short_sha = if sha.len() > 8 { &sha[..8] } else { sha };
            let diff_path = format!("{}/{}.diff", output, short_sha);
            std::fs::write(&diff_path, &diff)
                .with_context(|| format!("Failed to write diff to '{diff_path}'"))?;
            println!(
                "{} {}",
                "[OK] Diff written to:".green().bold(),
                diff_path.yellow()
            );
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn dump_all_commits(branch: &str, format: &str, output: &str) -> Result<()> {
    let default_branch = detect_default_branch()?;

    match format {
        "patch" => {
            println!("{}", "Generating patches for all branch commits...".cyan());
            let range = format!("{}..{}", default_branch, branch);
            let result = run_git(&["format-patch", &range, "-o", output])
                .with_context(|| format!("Failed to generate patches for '{branch}'"))?;
            if result.is_empty() {
                println!(
                    "{}",
                    "[WARN] No commits found between the default branch and the target."
                        .yellow()
                        .bold()
                );
            } else {
                for line in result.lines() {
                    println!("  {}", line);
                }
                println!("{}", "[OK] All patches generated.".green().bold());
            }
        }
        "diff" => {
            println!("{}", "Generating diff for all branch commits...".cyan());
            let diff = run_git(&["diff", &default_branch, branch])
                .with_context(|| format!("Failed to generate diff for '{branch}'"))?;
            if diff.is_empty() {
                println!(
                    "{}",
                    "[WARN] No diff between the default branch and the target."
                        .yellow()
                        .bold()
                );
            } else {
                let safe_name = branch.replace('/', "-");
                let diff_path = format!("{}/{}.diff", output, safe_name);
                std::fs::write(&diff_path, &diff)
                    .with_context(|| format!("Failed to write diff to '{diff_path}'"))?;
                println!(
                    "{} {}",
                    "[OK] Diff written to:".green().bold(),
                    diff_path.yellow()
                );
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn send_patches(output: &str, email: &str) -> Result<()> {
    println!(
        "{} {}",
        "Sending patches via git send-email to:".cyan(),
        email.yellow()
    );

    run_git(&["send-email", "--to", email, output])
        .context("Failed to send patches via git send-email. Is git-send-email installed?")?;

    println!("{}", "[OK] Patches sent successfully.".green().bold());
    Ok(())
}
