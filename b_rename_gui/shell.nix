{ pkgs ? import <nixpkgs> { } }:

let
  # 创建一个包含系统环境的 shell
  systemShell = pkgs.mkShell {
    buildInputs = [ ];
  };
in
pkgs.mkShell {
  # 继承系统环境
  inputsFrom = [ systemShell ];

  # 添加项目特定的包
  buildInputs = with pkgs; [
    dbus
  ];

  nativeBuildInputs = with pkgs.buildPackages; [
    pkg-config
    dbus
    glib
    pango
    gdk-pixbuf
    graphene
    gtk4
    libadwaita
    rustc
    cargo
  ];

  # 设置环境变量（如果需要）
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  # shell 钩子
  shellHook = ''
    echo "进入 Rust 开发环境，包含系统环境和项目特定包"
    echo "Rust 版本: $(rustc --version)"
    echo "Cargo 版本: $(cargo --version)"
  '';
}
