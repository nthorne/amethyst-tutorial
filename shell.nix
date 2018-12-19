{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = with pkgs; [
      # amethyst dependencies
      alsaLib
      cmake
      expat
      freetype
      openssl
      pkgconfig
      xorg.libxcb

      entr
      rustup
      rustracer
      tmuxinator
    ];
}

