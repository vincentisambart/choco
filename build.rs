fn main() {
    cc::Build::new()
        .file("src/choco.mm")
        .flag("-fobjc-arc")
        .flag("-std=c++17")
        .flag("-Wall")
        .flag("-Wextra")
        .compile("choco");
    println!("cargo:rustc-link-lib=framework=Foundation")
}
