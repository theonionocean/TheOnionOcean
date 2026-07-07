{pkgs ? import <nixpkgs> {}}: let
  # Pre-built binary: compiling from source fails on CPUs without AVX-512 due to an
  # LLVM bug in diskann-vector's _mm512_dpwssd_epi32 intrinsic (rustc 1.96 / LLVM 21).
  surrealkit = pkgs.stdenv.mkDerivation rec {
    pname = "surrealkit";
    version = "0.7.0";
    src = pkgs.fetchurl {
      url = "https://github.com/surrealdb/surrealkit/releases/download/v${version}/surrealkit-v${version}-x86_64-unknown-linux-gnu.tar.gz";
      hash = "sha256-aG1X2qwT6p88K3U9jU7HXdx4uLru+KiXe79iEQkYYSI=";
    };
    dontUnpack = true;
    installPhase = ''
      runHook preInstall
      tar -xzf $src surrealkit
      install -Dm755 surrealkit $out/bin/surrealkit
      runHook postInstall
    '';
  };
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
      surrealkit
    ];

    shellHook = ''
      export NX_SKIP_FORMAT=true
      export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
      export LIBCLANG_PATH="${pkgs.lib.makeLibraryPath [pkgs.llvmPackages.libclang.lib]}"
    '';
  }
