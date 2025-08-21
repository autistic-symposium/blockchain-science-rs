{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "rust-env";
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.clippy
    pkgs.cargo-audit
    pkgs.rust-analyzer
  ];
}

