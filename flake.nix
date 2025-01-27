{
  description = "A basic Rust devshell for NixOS users";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
    nix-filter.url = "github:numtide/nix-filter";
    };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, nix-filter, ... }:
    with flake-utils.lib; eachSystem allSystems (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        sources = {
          test = nix-filter.lib {
            root = ./.;
            include = [
              ./Cargo.lock
              ./test.sh
              (nix-filter.lib.inDirectory "src")
              (nix-filter.lib.inDirectory "tests")
              (nix-filter.lib.matchExt "toml")
              (nix-filter.lib.matchExt "lock")
            ];
          }; 
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            cacert
            cargo-make
            cargo-bump
            trunk
            fontconfig
            (rust-bin.selectLatestNightlyWith( toolchain: toolchain.default.override {
              extensions= [ "rust-src" "rust-analyzer" ];
            }))
          ] ++ pkgs.lib.optionals pkg.stdenv.isDarwin [
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];

        };
      }
    );
}
