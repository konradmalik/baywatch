{
  description = "baywatch";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, ... }:
    let
      forAllSystems = function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
          "x86_64-darwin"
          "aarch64-darwin"
        ]
          (system:
            function (import nixpkgs {
              inherit system;
              config.allowUnfree = true;
            }));
      darwinDeps = pkgs: with pkgs; [
        darwin.apple_sdk.frameworks.CoreServices
        libiconv
      ];
    in
    {
      devShells = forAllSystems (pkgs: {
        default =
          pkgs.mkShell
            {
              name = "bwatch";
              packages = with pkgs; [
                cargo
                cargo-edit
                clippy
                rustc
              ] ++
              (lib.optional pkgs.stdenvNoCC.isDarwin (darwinDeps pkgs));
            };
      });
      formatter = forAllSystems (pkgs: pkgs.nixpkgs-fmt);
      packages = forAllSystems (pkgs:
        {
          bwatch = with pkgs;
            let
              cargoToml = with builtins; (fromTOML (readFile ./Cargo.toml));
              pname = cargoToml.package.name;
              version = cargoToml.package.version;
              cargoLock.lockFile = ./Cargo.lock;
              darwinBuildInputs = (darwinDeps pkgs);
            in
            pkgs.rustPlatform.buildRustPackage {
              inherit pname version cargoLock;
              src = lib.cleanSource ./.;
              nativeBuildInputs = [ clippy rustfmt ];
              buildInputs = [ ] ++ lib.optionals stdenv.isDarwin darwinBuildInputs;
              preBuildPhases = [ "cargoFmt" ];
              cargoFmt = ''
                cargo fmt --manifest-path ./Cargo.toml --all --check
              '';
              # right after checkPhase (tests)
              preInstallPhases = [ "clippy" ];
              clippy = ''
                cargo clippy -- --deny warnings
              '';
            };
          default = self.packages.${pkgs.system}.bwatch;
        });
      apps = forAllSystems (pkgs: {
        default = {
          type = "app";
          program = "${self.packages.${pkgs.system}.bwatch}/bin/bwatch";
        };
      });
    };
}
