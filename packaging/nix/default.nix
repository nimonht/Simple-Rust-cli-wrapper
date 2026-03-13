{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "git-workflow";
  version = "1.0.1";

  # To build from the local source tree instead of fetching from GitHub,
  # replace the src block below with:
  #   src = /path/to/Simple-Rust-cli-wrapper;
  src = pkgs.fetchFromGitHub {
    owner = "nimonht";
    repo = "Simple-Rust-cli-wrapper";
    rev = "v${version}";
    # After tagging a release, compute the hash with:
    #   nix-prefetch-url --unpack \
    #     https://github.com/nimonht/Simple-Rust-cli-wrapper/archive/v<VERSION>.tar.gz
    # Then convert the base32 output to SRI format with:
    #   nix hash to-sri --type sha256 <BASE32_HASH>
    hash = pkgs.lib.fakeHash;
  };

  # After the first build attempt with fakeHash, Nix will print the expected
  # cargoHash in the error output.  Paste it here to replace fakeHash.
  # Alternatively, generate it with:
  #   nix-prefetch { inherit src; } (using cargo-hash tooling)
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
}
