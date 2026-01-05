{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    just
    bacon
    presenterm
    # FFI dependencies
    pkg-config
    glib
  ];

  shellHook = ''
    rustup update
  '';
}