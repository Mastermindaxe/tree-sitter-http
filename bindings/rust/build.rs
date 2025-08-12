fn main() {
    let root_dir = std::path::Path::new(".");
    let http_dir = root_dir.join("src");

    let mut config = cc::Build::new();
    config.include(&http_dir);
    config
        .flag_if_supported("-std=c11")
        .flag_if_supported("-Wno-unused-parameter");

    for path in &[
        http_dir.join("parser.c"),
    ] {
        config.file(path);
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    config.compile("tree-sitter-http");
}
