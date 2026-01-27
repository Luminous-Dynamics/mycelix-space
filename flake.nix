{
  description = "Mycelix Space - Decentralized Space Domain Awareness Network";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    # Holonix - official Holochain development environment
    holonix = {
      url = "github:holochain/holonix?ref=main";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, holonix, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain with wasm target
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

      in
      {
        # Development shell with Holochain tools from holonix
        devShells.default = pkgs.mkShell {
          inputsFrom = [
            holonix.devShells.${system}.default
          ];
          buildInputs = with pkgs; [
            # Additional tools
            cargo-watch
            cargo-expand
            nodejs_20
            nodePackages.npm
          ];
          shellHook = ''
            echo "=== Mycelix Space Development Environment ==="
            echo ""
            echo "Rust: $(rustc --version 2>/dev/null || echo 'checking...')"
            if command -v hc &> /dev/null; then
              echo "Holochain CLI: $(hc --version 2>/dev/null || echo 'available')"
            else
              echo "Holochain CLI: not available"
            fi
            echo ""
            echo "Commands:"
            echo "  ./scripts/build-happ.sh    - Build WASM and package hApp"
            echo "  hc sandbox generate        - Create test sandbox"
            echo "  hc sandbox run             - Run sandbox conductor"
            echo "  cargo check --workspace    - Check all code"
            echo "  cargo test --workspace     - Run tests"
            echo ""
            echo "UI Development:"
            echo "  cd ui && npm install && npm run dev"
            echo ""
          '';
        };

        # Rust-only shell (faster to enter, for pure Rust development)
        devShells.rust = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo-watch
            cargo-expand
            pkg-config
            openssl
          ];
          shellHook = ''
            echo "=== Mycelix Space Rust Development ==="
            echo "Rust: $(rustc --version)"
            echo ""
            echo "Note: This shell doesn't include Holochain CLI."
            echo "Use 'nix develop' for full Holochain tooling."
            echo ""
          '';
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        # UI development shell (Node.js only)
        devShells.ui = pkgs.mkShell {
          buildInputs = with pkgs; [
            nodejs_20
            nodePackages.npm
          ];
          shellHook = ''
            echo "=== Mycelix Space UI Development ==="
            echo "Node: $(node --version)"
            echo "npm: $(npm --version)"
            echo ""
            echo "Commands:"
            echo "  cd ui && npm install"
            echo "  npm run dev    - Start dev server"
            echo "  npm run build  - Production build"
            echo ""
          '';
        };

        # Package the orbital-mechanics library
        packages.orbital-mechanics = pkgs.rustPlatform.buildRustPackage {
          pname = "orbital-mechanics";
          version = "0.1.0";
          src = ./lib/orbital-mechanics;
          cargoLock.lockFile = ./Cargo.lock;
        };
      }
    );
}
