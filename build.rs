fn main() {
    cc::Build::new()
        .file("src/choco.mm")
        .flag("-fobjc-arc")
        .flag("-std=c++17")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Werror=conversion")
        .compile("choco");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=AVFoundation");
}
