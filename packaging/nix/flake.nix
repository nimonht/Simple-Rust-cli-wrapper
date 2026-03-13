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
          version = "1.0.0-rc1";

          # To build from a published release, use fetchFromGitHub:
          #
          #   src = pkgs.fetchFromGitHub {
          #     owner = "nimonht";
          #     repo = "Simple-Rust-cli-wrapper";
          #     rev = "v${version}";
          #     hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
          #   };
          #
          # To obtain the real hash, run:
          #   nix-prefetch-url --unpack \
          #     https://github.com/nimonht/Simple-Rust-cli-wrapper/archive/v${version}.tar.gz
          #
          # Then convert to SRI format:
          #   nix hash to-sri --type sha256 <hex-hash>

          # Default: build from local source (for development and local installs).
          # Point this at the repo root when building locally:
          #   nix build .#git-workflow --override-input self /path/to/repo
          src = pkgs.fetchFromGitHub {
            owner = "nimonht";
            repo = "Simple-Rust-cli-wrapper";
            rev = "v${version}";
            # Replace with the real hash after tagging a release. To compute:
            #   nix-prefetch-url --unpack \
            #     https://github.com/nimonht/Simple-Rust-cli-wrapper/archive/v<VERSION>.tar.gz
            #   nix hash to-sri --type sha256 <hex-hash>
            hash = pkgs.lib.fakeHash;
          };

          # Replace with the real Cargo dependency hash after first build.
          # Build once, let it fail, then copy the expected hash from the
          # error message and paste it here.
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

        # Build from local source tree -- useful for development and for users
        # who cloned the repo and want to install via their package manager
        # without waiting for a published release.
        git-workflow-local = pkgs.rustPlatform.buildRustPackage {
          pname = "git-workflow";
          version = "1.0.0-rc1";

          src = self;

          cargoLock = {
            lockFile = self + "/Cargo.lock";
          };

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
          default = git-workflow-local;
          inherit git-workflow;
          local = git-workflow-local;
        };

        apps.default = {
          type = "app";
          program = "${git-workflow-local}/bin/git-workflow";
        };
      }
    );
}
