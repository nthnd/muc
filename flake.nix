{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        parts.follows = "parts";
      };
    };
  };

  outputs = inputs@{ parts, ... }:
    parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.nci.flakeModule
        parts.flakeModules.easyOverlay
      ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = { config, ... }:
        let crateOutputs = config.nci.outputs.muc; in
        {
          nci.projects.muc.path = ./.;
          overlayAttrs.muc = config.packages.default;
          packages.default = crateOutputs.packages.release;
          devShells.default = crateOutputs.devShell;
        };
    };
}
