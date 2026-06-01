{
  description = "A tutorial on the use of WebAssembly in the Compute Continuum";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    devshell.url = "github:numtide/devshell";
    utils.url = "git+https://github.com/numtide/flake-utils.git";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    devshell,
    utils,
    ...
  }@inputs:
  {
    overlays.default = import ./overlay.nix;
  };

  utils.lib.eachSystem
  [
    "x86_64-linux"
  ]
  (
    system:
    let 
      lib = nixpkgs.lib;
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          # import oxalica's rust overlay for version specific Rust toolchains
          (import inputs.rust-overlay)

          # inject dependencies for our overlay.nix
          (final: prev: {
          inherit (inputs.typix.lib.${system}) buildTypstProject;
          })

          # import our overlay for the package in pkgs/
          self.overlays.default

          # add the devshell overlay for the devshell defined below
          devshell.overlays.default
        ];
      };

      # rust target name of the `system`
      rust-target = pkgs.pkgsStatic.stdenv.targetPlatform.rust.rustcTarget;

      # parsed contents of Cargo.toml
      cargoToml = lib.trivial.importTOML ./Cargo.toml;

      # minimum rust version that we support according to Cargo.toml
      msrv = cargoToml.package.rust-version;

      # Rust distribution for our hostSystem
      rust-toolchain-nixpkgs-current = pkgs.rust-bin.stable.${pkgs.rustc.version}.default.override {
        extensions = [ "rust-src" ];
          targets = [
            rust-target
            "wasm32-unknown-unknown"
            "thumbv6m-none-eabi" # for no_std test
            "i686-unknown-linux-musl" # to test if we can run on 32 Bit architectures
          ];
      };

      # Rust toolchain for the MSRV
      rust-toolchain-msrv = pkgs.rust-bin.stable.${msrv}.default.override {
        extensions = [ "rust-src" ];
          targets = [
            rust-target
            "wasm32-unknown-unknown"
            "thumbv6m-none-eabi" # for no_std test
            "i686-unknown-linux-musl" # to test if we can run on 32 Bit architectures
          ];
      };
    in
    {
      devshell.default = (
        packages = with pkgs; [
          stdenv.cc
          coreutils
          rust-toolchain-nixpkgs-current # also contains clippy
          rust-analyzer
          cargo-watch
        ];
      )
    }
  )

}