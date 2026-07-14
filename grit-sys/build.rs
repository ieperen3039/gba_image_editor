use std::env;
use std::path::PathBuf;

fn main() {
    // Compile Grit's C++ sources
    cc::Build::new()
        .cpp(true)
        .include("include")
        .define("VERSION_STRING", "\"v1.21.1\"")
        .file("src/cprs.cpp")
        .file("src/cprs_huff.cpp")
        .file("src/cprs_lz.cpp")
        .file("src/cprs_rle.cpp")
        .file("src/grit_core.cpp")
        .file("src/grit_misc.cpp")
        .file("src/grit_prep.cpp")
        .file("src/grit_shared.cpp")
        .file("src/grit_xp.cpp")
        .file("src/logger.cpp")
        .file("src/pathfun.cpp")
        .compile("grit");

    // Link the static library
    println!("cargo:rustc-link-lib=static=grit");

    // Link C++ standard library (necessary for C++ code on Unix-like systems)
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // Generate bindings from Grit's C headers
    let bindings = bindgen::Builder::default()
        .header("include/grit.h")
        .clang_args(["-x", "c++"])
        // Add other headers as needed
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
