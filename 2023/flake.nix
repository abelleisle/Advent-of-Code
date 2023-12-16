{
  description = "Advent of code 2023";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs:
    let
      inherit (inputs) nixpkgs flake-utils;
      systems = ["x86_64-linux"];
    in
    flake-utils.lib.eachSystem systems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      rec {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            zig
            zls
          ];

          shellHook = ''
            zsh
            exit # Automatically exit the bash shell after zsh exits
          '';
        };
      }
    );
}
