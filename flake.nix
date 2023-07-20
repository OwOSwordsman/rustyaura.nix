{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    naersk,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # https://github.com/oxalica/rust-overlay#cheat-sheet-common-usage-of-rust-bin
        toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src"];
        };

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

        sharedDependencies = with pkgs; [toolchain sccache];
        linuxDependencies = with pkgs; [mold clang];
        macosDependencies = with pkgs; [];
        macosFrameworks = with pkgs.darwin.apple_sdk.frameworks; [];

        dependencies =
          sharedDependencies
          ++ pkgs.lib.optionals pkgs.stdenv.isLinux linuxDependencies
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin macosDependencies
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin macosFrameworks;
      in rec {
        packages.default = packages.rustyaura;

        packages.rustyaura = naersk'.buildPackage {
          src = ./.;
          buildInputs = dependencies ++ [pkgs.makeWrapper];

          postInstall = ''
            wrapProgram "$out/bin/rustyaura" --prefix PATH : "${packages.prettierd-tailwind}/bin"
          '';
        };

        packages.prettierd-tailwind = pkgs.buildNpmPackage {
          pname = "prettierd";
          version = "0.1.0";

          src = ./.;
          npmDepsHash = "sha256-NfTU0+gP3Zwm2IDvIHEm00KCmyR47Fpolw6yEMTSE4c=";
          dontNpmBuild = true;

          nativeBuildInputs = [pkgs.makeWrapper];
          postInstall = ''
            mkdir -p $out/bin
            makeWrapper "$out/lib/node_modules/prettierd-tailwind/node_modules/@fsouza/prettierd/bin/prettierd" \
              $out/bin/prettierd
          '';
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; dependencies ++ [nodejs];
          env = {
            LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath dependencies;
          };
        };
      }
    );
}
