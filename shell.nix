{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = with pkgs; [
      # amethyst dependencies
      alsaLib
      cmake
      expat
      freetype
      libGL
      openssl
      pkgconfig
      xorg.libX11
      xorg.libxcb
      xorg.libXi
      xorg.libXtst
      xorg.libXcursor
      xorg.libXrandr

      entr
      rustup
      rustracer
      tmuxinator
    ];

    # TODO: There must be a nicer way to do this..
    shellHook =  ''
      export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.xorg.libX11}/lib:${pkgs.xorg.libXtst}/lib";
      export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.xorg.libXcursor}/lib:${pkgs.xorg.libXrandr}/lib";
      export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.xorg.libXi}/lib";
      export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.libGL}/lib";
      '';
}

