{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk-lib = pkgs.callPackage naersk {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };

        ignoreSource = [ ".github" "*.nix" ];
        backend = naersk-lib.buildPackage {
          inherit nativeBuildInputs buildInputs;
          pname = "backend";
          src = pkgs.nix-gitignore.gitignoreSource ignoreSource ./.;
        };

        dockerImage =
          let
            defaultPort = "8080";
          in
          pkgs.dockerTools.streamLayeredImage {
            name = "backend";
            tag = "${self.lastModifiedDate}-${self.shortRev or "dirty"}";
            config = {
              Cmd = [
                "${backend}/bin/backend"
              ];
              ExposedPorts = {
                "${defaultPort}/tcp" = { };
              };
              Env = [ 
                "PORT=${defaultPort}"
                "DATABASE_URL=postgres://username:password@address/database"
              ];
            };
          };

        nativeBuildInputs = with pkgs; [ clang ];
        buildInputs = with pkgs; [ postgresql.lib ];
      in
      {
        packages = {
          inherit backend dockerImage;
        };
        packages.default = self.packages.${system}.backend;

        devShells.default = with pkgs; mkShell {
          nativeBuildInputs = nativeBuildInputs ++ [
            diesel-cli
          ];
          buildInputs = buildInputs ++ [
            (rust-toolchain.override { extensions = [ "rust-src" ]; })
          ];
        };

        checks = {
          backend = self.packages.${system}.default.overrideAttrs (super: { doCheck = true; });
        };
      });

  nixConfig = {
    extra-experimental-features = "nix-command flakes";
    extra-substituters = [ "https://elxreno-rust.cachix.org" ];
    extra-trusted-public-keys = [ "elxreno-rust.cachix.org-1:cfUElkBCai6A6hqku/tOCrYt9qF+vQtAV8+8MF16gf8=" ];
  };
}
