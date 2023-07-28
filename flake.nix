{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in
      rec {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            botan2
            cargo
            gcc
            just
            netcat-gnu
            nodePackages.pnpm
            nodejs_18
            rustc
            rustup
            sqlite
            sqlx-cli
          ];
        };
      });
}
