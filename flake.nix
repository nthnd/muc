{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

  };

  outputs = { self, nixpkgs, flake-utils, flake-compat }: {
    overlays.default = _: prev:
      let
        inherit (prev.rustPlatform) buildRustPackage;
        toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in
      {
        muc = buildRustPackage {
          pname = "muc";
          src = self;
          inherit (toml.package) version;
          cargoHash = "sha256-w/b4qps4z1HrtSlSklflU6i1QfSFQw20+uALIpCIk8I=";
        };
      };
  } //
  (flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ self.overlays.default ];
      };
      inherit (pkgs) muc;
    in
    {
      packages = {
        inherit muc;
        default = muc;
      };
    }));
}
