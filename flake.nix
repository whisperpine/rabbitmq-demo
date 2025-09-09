{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                rust-overlay.overlays.default
                self.overlays.default
              ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          let
            rust = prev.rust-bin;
          in
          # rust.stable.latest.default.override {
          #   extensions = [
          #     "rust-src"
          #     "llvm-tools" # required by cargo-llvm-cov
          #   ];
          #   targets = [ ];
          # };
          rust.nightly."2025-06-20".default.override {
            extensions = [
              "rust-src"
              "llvm-tools" # required by cargo-llvm-cov
            ];
            targets = [ ];
          };
      };

      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              cargo-edit # managing cargo dependencies
              cargo-nextest # next-generation test runner
              bacon # background code checker
              just # just a command runner
              husky # manage git hooks
            ];
            shellHook = ''
              # install git hook managed by husky
              if [ ! -e "./.husky/_" ]; then
                husky install
              fi
            '';
          };
        }
      );
    };
}
