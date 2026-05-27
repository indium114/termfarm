{
  description = "go devshell and package, created by scaffolder";

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
          name = "go-devshell";

          packages = with pkgs; [
            go
            gopls
            gotools
            delve
            just
          ];
        };

        packages.example = pkgs.buildGoModule {
          pname = "termfarm";
          version = "2026.05.27-c";

          src = self;

          vendorHash = pkgs.lib.fakeHash; # TODO: Change

          subPackages = [ "." ];
          ldflags = [ "-s" "-w" ];

          meta = with pkgs.lib; {
            description = "A CLI idle farming game";
            license = licenses.mit;
            platforms = platforms.all;
          };
        };

        apps.example = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.example}/bin/termfarm";
        };
      });
}
