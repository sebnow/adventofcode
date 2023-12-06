{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = {
    self,
    nixpkgs,
    devenv,
    systems,
    ...
  } @ inputs: let
    forEachSystem = nixpkgs.lib.genAttrs (import systems);
  in {
    packages = forEachSystem (system: {
      devenv-up = self.devShells.${system}.default.config.procfileScript;
    });

    formatter = forEachSystem (system: nixpkgs.legacyPackages.${system}.alejandra);
    devShells =
      forEachSystem
      (system: let
        pkgs = nixpkgs.legacyPackages.${system};
        ocamlPackages = pkgs.ocaml-ng.ocamlPackages;
      in {
        default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              languages.rust.enable = true;
              packages = [pkgs.heaptrack];
            }
            rec {
              languages.ocaml = {
                enable = true;
                packages = pkgs.ocaml-ng.ocamlPackages_4_12;
              };
              packages = [
                pkgs.opam
              ];

              enterShell = ''
                eval $(opam env --switch=default)
              '';
            }
          ];
        };
      });
  };
}
