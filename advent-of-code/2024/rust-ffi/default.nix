{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    justfile
    bacon
    presenterm
  ];

  shellHook = ''
    rustup update
  '';
}