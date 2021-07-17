{

  description = "";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build`
          packages.rlox = naersk-lib.buildPackage {
            pname = "rlox";
            root = ./.;
          };
          defaultPackage = packages.rlox;

          # `nix run`
          apps.rlox = flake-utils.lib.mkApp {
            drv = packages.rlox;
          };
          defaultApp = apps.rlox;

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ rustc cargo ];
          };
        }
    );
}
