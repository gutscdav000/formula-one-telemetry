{
  description = "Formula One Telemetry Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Use the latest stable Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            redis
            cargo
            rustc
            rustfmt
            clippy
          ];

          shellHook = ''
            echo "🚀 Formula One Telemetry Development Environment"
            echo "📦 Rust: $(rustc --version)"
            echo "🔴 Redis CLI: $(redis-cli --version)"
            echo ""
            echo "Available tools:"
            echo "  - rustc, cargo, rustfmt, clippy"
            echo "  - redis-cli"
          '';
        };
      }
    );
}

