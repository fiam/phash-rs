use cmake::Config;

#[cfg(target_os = "macos")]
fn platform_libdir() {
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
}

#[cfg(not(target_os = "macos"))]
fn platform_libdir() {
    
}

fn main() {
    let dst = Config::new("pHash")
        .define("PHASH_DYNAMIC", "OFF")
        .define("PHASH_STATIC", "ON")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=pHash");

    println!("cargo:rustc-flags=-l dylib=c++");
    println!("cargo:rustc-flags=-l dylib=png");
    println!("cargo:rustc-flags=-l dylib=jpeg");
    println!("cargo:rustc-flags=-l dylib=tiff");

    platform_libdir();
}
