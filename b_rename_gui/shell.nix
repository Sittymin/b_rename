{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    dbus
    gsettings-desktop-schemas
    glib
    gtk4
    libadwaita
    pkg-config
    rustc
    cargo
  ];

  shellHook = ''
    echo "进入 Rust GTK4 开发环境"
  
    export GSETTINGS_SCHEMA_DIR=$(mktemp -d)
  
    # echo "gsettings-desktop-schemas 路径: ${pkgs.gsettings-desktop-schemas}"
    # echo "gtk4 路径: ${pkgs.gtk4}"
  
    # 只复制 .xml 文件，不复制 gschemas.compiled
    find ${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}/glib-2.0/schemas -name '*.xml' -exec cp {} $GSETTINGS_SCHEMA_DIR/ \;
    find ${pkgs.gtk4}/share/gsettings-schemas/${pkgs.gtk4.name}/glib-2.0/schemas -name '*.xml' -exec cp {} $GSETTINGS_SCHEMA_DIR/ \;
  
    # 编译所有 schema
    glib-compile-schemas $GSETTINGS_SCHEMA_DIR
  
    export XDG_DATA_DIRS="$GSETTINGS_SCHEMA_DIR:$XDG_DATA_DIRS"
  
    # echo "GSettings schema 目录: $GSETTINGS_SCHEMA_DIR"
  '';
}
