{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    dbus
  ];
  # nativeBuildInputs is usually what you want -- tools you need to run
  nativeBuildInputs = with pkgs.buildPackages; [
    pkg-config
    dbus
    glib
    pango
    gdk-pixbuf
    graphene
    gtk4
  ];

}
