# Copyright 2024 Gentoo Authors
# Distributed under the terms of the BSD-2 license

EAPI=8

inherit cargo

DESCRIPTION="A CLI tool that automates Git workflows"
HOMEPAGE="https://github.com/nimonht/Simple-Rust-cli-wrapper"
SRC_URI="https://github.com/nimonht/Simple-Rust-cli-wrapper/archive/v${PV}.tar.gz -> ${P}.tar.gz"

S="${WORKDIR}/Simple-Rust-cli-wrapper-${PV}"

LICENSE="BSD-2"
SLOT="0"
KEYWORDS="~amd64"

BDEPEND="virtual/rust"
RDEPEND="
	dev-vcs/git
	dev-util/github-cli
"

src_install() {
	cargo_src_install
}
