fn main() {
    glib_build_tools::compile_resources(
        &["src/resources"],                      // 资源文件夹的路径
        "src/resources/resources.gresource.xml", // GResource XML 文件的路径
        "b_rename.gresource",                    // 输出的 GResource 文件名
    );
}

