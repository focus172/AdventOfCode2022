{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
  };

  outputs = { self, nixpkgs, utils, }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "aoc-2022";
          version = "0.1.0";

          src = ./.;

          nativeBuildInputs = with pkgs; [ zulu ];
          buildInputs = with pkgs; [ ];

          # shellHook = ''export VIMRUNTIME=$PWD/runtime'';
        };
      });
}

