{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "git-workflow";
  version = "0.2.0";

  src = pkgs.fetchFromGitHub {
    owner = "nimonht";
    repo = "Simple-Rust-cli-wrapper";
    rev = "v${version}";
    hash = ""; # Replace with actual hash after first build
  };

  cargoHash = ""; # Replace with actual hash after first build

  nativeBuildInputs = [ pkgs.makeWrapper ];

  postInstall = ''
    wrapProgram $out/bin/git-workflow \
      --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.git pkgs.gh ]}
  '';

  meta = with pkgs.lib; {
    description = "A CLI tool that automates Git workflows";
    homepage = "https://github.com/nimonht/Simple-Rust-cli-wrapper";
    license = licenses.bsd2;
    maintainers = [];
    mainProgram = "git-workflow";
  };
}
