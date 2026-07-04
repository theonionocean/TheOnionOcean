{pkgs ? import <nixpkgs> {}}: let
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      alejandra
      nodejs_24
      bun
      biome
      cargo
      clang
      clippy
      cmake
      llvmPackages.libclang
      pkg-config
      rust-analyzer
      rustfmt
      rustc
      rustPlatform.rustLibSrc
      just
    ];

    shellHook = ''
      export NX_SKIP_FORMAT=true
      export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
      export LIBCLANG_PATH="${pkgs.lib.makeLibraryPath [pkgs.llvmPackages.libclang.lib]}"
    '';
  }
