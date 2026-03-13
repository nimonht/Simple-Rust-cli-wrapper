# RPM spec file for git-workflow
# https://github.com/nimonht/Simple-Rust-cli-wrapper
#
# Build:
#   rpmbuild -ba git-workflow.spec

%global crate git-workflow
%global repo_name Simple-Rust-cli-wrapper
# Upstream version may contain hyphens (e.g. 1.0.0-rc1) which RPM does not
# allow in Version.  The tilde form (1.0.0~rc1) is set in Version below.
# upstream_version preserves the original for tarball URLs and directory names.
%global upstream_version 1.0.0

Name:           git-workflow
Version:        1.0.0
Release:        1%{?dist}
Summary:        A CLI tool that automates Git workflows

License:        BSD-2-Clause
URL:            https://github.com/nimonht/%{repo_name}
Source0:        %{url}/archive/v%{upstream_version}/%{repo_name}-%{upstream_version}.tar.gz

ExclusiveArch:  x86_64 aarch64

BuildRequires:  rust
BuildRequires:  cargo

Requires:       git
Requires:       gh

%description
git-workflow is a Rust CLI tool that automates common Git workflows.
It provides subcommands to start feature branches, finish work with
automatic staging/commit/push/PR creation via GitHub CLI, dump commits
to patch or diff files, and an interactive TUI mode.

Subcommands:
  start   - Sync the default branch and create a new feature branch.
  finish  - Stage all changes, commit, push, and open a Pull Request.
  dump    - Dump commits from a branch to patch or diff files.
  tui     - Launch interactive TUI mode.

%prep
%autosetup -n %{repo_name}-%{upstream_version}

%build
export CARGO_HOME="%{_builddir}/.cargo"
cargo build --release %{?_smp_mflags}

%install
install -D -p -m 0755 target/release/%{crate} %{buildroot}%{_bindir}/%{crate}
install -D -p -m 0644 LICENSE %{buildroot}%{_licensedir}/%{crate}/LICENSE

%check
export CARGO_HOME="%{_builddir}/.cargo"
cargo test %{?_smp_mflags}

%files
%license LICENSE
%{_bindir}/%{crate}

%changelog
* Thu Jul 03 2025 nimonht <nimonht on GitHub> - 1.0.0~rc1-1
- Initial RPM package
