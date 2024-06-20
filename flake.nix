{
  description = "Description for the project";

  inputs = {
    nixgl.url = "github:nix-community/nixGL";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    boulder.url = "github:berkeleytrue/nix-boulder-banner";
  };

  outputs = inputs @ {
    flake-parts,
    nixpkgs,
    nixgl,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.boulder.flakeModule
      ];
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        config,
        system,
        ...
      }: let
        pkgs = import nixpkgs {
          inherit system;

          overlays = [
            inputs.nixgl.overlay
          ];
        };
        run = pkgs.writeShellScriptBin "run" ''
          echo "nix cargo run"
          ${pkgs.nixgl.auto.nixGLDefault}/bin/nixGL cargo run
        '';
      in {
        formatter.default = pkgs.alejandra;
        boulder.commands = [
          {
            exec = run;
            description = "cargo run";
          }
        ];
        devShells.default = pkgs.mkShell {
          name = "slint-test";
          inputsFrom = [
            config.boulder.devShell
          ];

          buildInputs =
            (with pkgs; [
              cargo
              rustc
              libGL
              qt5.full
              ffmpeg
            ]);
        };
      };
      flake = {};
    };
}
