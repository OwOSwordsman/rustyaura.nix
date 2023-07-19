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
          buildInputs = dependencies ++ [packages.prettier];
        };

        packages.prettier-plugin-tailwindcss = pkgs.buildNpmPackage rec {
          pname = "prettier-plugin-tailwindcss";
          version = "0.4.1";

          src = pkgs.fetchFromGitHub {
            owner = "tailwindlabs";
            repo = pname;
            rev = "v${version}";
            hash = "sha256-yc434+Yhhzw1ivz+oAgLCPCndVb0g+KucPxPknGlRp4=";
          };

          npmDepsHash = "sha256-b2ioMu+9+j6xPe+0hqu9JrZfVW5eH4b3HISeTYeugeY=";
        };

        packages.prettier = pkgs.stdenv.mkDerivation {
          name = "prettier";

          src = ./.;
          nativeBuildInputs = with pkgs; [makeWrapper];

          buildPhase = ''
            mkdir -p $out/lib/node_modules
            cp -r ${pkgs.nodePackages.prettier}/lib/node_modules/* $out/lib/node_modules
            cp -r ${packages.prettier-plugin-tailwindcss}/lib/node_modules/* $out/lib/node_modules
          '';

          installPhase = ''
            mkdir -p $out/bin
            makeWrapper $out/lib/node_modules/prettier/bin-prettier.js $out/bin/prettier \
              --add-flags "--tab-width 4" \
              --add-flags "--plugin $out/lib/node_modules/prettier-plugin-tailwindcss" \
              --add-flags "--plugin $out/lib/node_modules/prettier"
          '';
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; dependencies ++ [packages.prettier];
          env = {
            LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath dependencies;
          };
        };
      }
    );
}
