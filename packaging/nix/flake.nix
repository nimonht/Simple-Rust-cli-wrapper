{
  description = "A CLI tool that automates Git workflows";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachSystem [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ] (system:
      let
        pkgs = import nixpkgs { inherit system; };

        git-workflow = pkgs.rustPlatform.buildRustPackage rec {
          pname = "git-workflow";
          version = "1.0.0";

          src = pkgs.fetchFromGitHub {
            owner = "nimonht";
            repo = "Simple-Rust-cli-wrapper";
            rev = "v${version}";
            hash = ""; # Replace with actual hash after first build
          };

          # Replace with actual hash after first build
          cargoHash = pkgs.lib.fakeHash;

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
        };
      in
      {
        packages = {
          default = git-workflow;
          inherit git-workflow;
        };

        apps.default = {
          type = "app";
          program = "${git-workflow}/bin/git-workflow";
        };
      }
    );
}
