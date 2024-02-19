{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    protobuf
    cmake
    pkg-config
    pdm
  ];

  LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";

  buildInputs = with pkgs; [
    openssl
    zlib
    libiconv
    grpc
  ];
}
