{
  description = "rust devshell and package, created by scaffolder";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          name = "rust-devshell";

          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            rust-analyzer
            clippy
            pkg-config
          ];
        };

        packages.termfarm = pkgs.rustPlatform.buildRustPackage {
          name = "termfarm";
          version = "2.0.1";

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;
        };

        apps.termfarm = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.termfarm}/bin/termfarm";
        };
      });
}
