{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        stableToolchain = pkgs.rust-bin.stable.latest.minimal.override {
          extensions = [ "clippy" "llvm-tools-preview" "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            openssl
            pkg-config
            stableToolchain
          ] ++ lib.optionals stdenv.isDarwin
            [ darwin.apple_sdk.frameworks.SystemConfiguration ];

          shellHook = ''
            # Using a local cargo home to avoid polluting the user's cargo home
            export CARGO_HOME=$PWD/.cargo
            export PATH=$CARGO_HOME/bin:$PATH
          '';
        };
      }
    );
}
