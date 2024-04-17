{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell rec{

  packages = with pkgs; [
    cargo
    rustup
  ];
}
