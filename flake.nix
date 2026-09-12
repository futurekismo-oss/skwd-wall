{
  description = "Skwd release binaries on NixOS";
  outputs = { self, nixpkgs }:
    let
      release = builtins.fromJSON (builtins.readFile ./nix/release.json);
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" ];
    in {
      packages = forAllSystems (system: import ./nix/binary-packages.nix {
        pkgs = import nixpkgs { inherit system; };
        inherit release;
      });
      nixosModules.default = import ./nix/nixos.nix { inherit self release; };
      checks = forAllSystems (system: import ./nix/checks.nix {
        pkgs = import nixpkgs { inherit system; };
        packages = self.packages.${system};
        nixosModule = self.nixosModules.default;
        inherit release;
      });
    };
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
}
