{pkgs ? import <nixpkgs> {}}: let
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      alejandra
      nodejs_24
      bun
      biome
      cargo
      clippy
      rust-analyzer
      rustfmt
      rustc
      rustPlatform.rustLibSrc
      just
    ];

    shellHook = ''
      export NX_SKIP_FORMAT=true
      export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
    '';
  }
