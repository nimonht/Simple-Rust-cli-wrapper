# Use Case Examples

## 1. Starting a new feature

You want to start working on a login page feature. Run:

```bash
git-workflow start feature/login-page
```

What happens behind the scenes:

1. Switches to your default branch (`master` or `main`).
2. Pulls the latest changes from the remote.
3. Creates and checks out a new branch called `feature/login-page`.

Output:

```
Syncing default branch...
Creating new branch: feature/login-page
[OK] Branch created and ready: feature/login-page
```

## 2. Finishing work and opening a Pull Request

You have made changes and are ready to submit. Run:

```bash
git-workflow finish "Add login page"
```

What happens behind the scenes:

1. Stages all changes (`git add .`).
2. Commits with the message `Add login page`.
3. Pushes the current branch to `origin`.
4. Opens a Pull Request on GitHub with the title `Add login page`.

Output:

```
Staging all changes (git add .)...
  Note: this stages ALL files in the working tree. Ensure .gitignore is configured correctly.
Committing with message: Add login page
Pushing branch: feature/login-page
Opening Pull Request...
[OK] Pull Request created: https://github.com/user/repo/pull/42
```

## 3. Bug fix workflow

```bash
# Start a branch for the fix
git-workflow start fix/null-pointer-crash

# ... make your code changes ...

# Commit and open a PR
git-workflow finish "Fix null pointer crash on startup"
```

## 4. Dumping patches from a branch

You are working on a kernel subsystem and need to export your commits as patch
files for mailing list review:

```bash
git-workflow dump --branch feature/driver-update --format patch --output ./patches
```

This creates one `.patch` file per commit in the `./patches` directory, ready
for `git am` or mailing list submission.

To dump as unified diff files instead:

```bash
git-workflow dump --branch feature/driver-update --format diff --output ./diffs
```

To dump a single commit:

```bash
git-workflow dump --commit abc1234 --format patch --output ./patches
```

To dump all commits on the current branch:

```bash
git-workflow dump --all --format patch --output ./patches
```

## 5. Sending patches via email

Export and email patches to a maintainer in one step:

```bash
git-workflow dump --branch feature/driver-update --format patch --email maintainer@example.com
```

## 6. Using the interactive TUI

Launch the terminal UI for an interactive menu-driven interface:

```bash
git-workflow tui
```

The TUI uses a lazydocker-style layout and respects your terminal's color theme.
Use keyboard navigation to select commands (Start Branch, Finish PR, Dump
Commits), fill in parameters, and execute actions.

## 7. Handling errors

If there are no changes to commit, the `finish` command reports the error
clearly:

```
Staging all changes (git add .)...
Committing with message: empty commit test
[ERROR] Failed to commit changes: git commit -m empty commit test failed: nothing to commit, working tree clean
```

If `gh` is not installed:

```
[ERROR] Failed to open Pull Request (is 'gh' installed and authenticated?): ...
```
