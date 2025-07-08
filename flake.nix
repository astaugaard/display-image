{
  description = "relm examples shell";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem =
        {
          self',
          pkgs,
          system,
          ...
        }:
        let
          rustVersion = "1.76.0";
          pkgs = import nixpkgs {
            inherit system;
          };
          runtimeDeps = with pkgs; [
            cairo
            gtk4
            atk
            glib
            gobject-introspection
            pango
            gdk-pixbuf
            graphene
            gtk4-layer-shell
          ];

        in
        {
          packages = rec {
            default = pkgs.dunst;
          };
          devShells.default = pkgs.mkShell rec {
            buildInputs =
              with pkgs;
              [
                pkg-config
              ]
              ++ runtimeDeps
              ++ [
                rust-analyzer
                cargo
                rustc
              ];
            LD_LIBRARY_PATH = "${nixpkgs.lib.makeLibraryPath buildInputs}";
          };
        };
    };
}
