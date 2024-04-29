{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachSystem ["x86_64-linux"] (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      kx = import ./default.nix {inherit pkgs;};
    in {
      packages.default =  kx;
      devShells.default = pkgs.mkShell {
        name = "kx";
        packages = [kx];
      };
    });
}
