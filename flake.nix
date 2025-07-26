{
  description = "Clojure template flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    edinix = {
      url = "github:fmarl/edinix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, edinix, ... }:
    flake-utils.lib.eachDefaultSystem (system: {
      devShells.default = import ./shell.nix { inherit system nixpkgs edinix; };
    }
  );
}
