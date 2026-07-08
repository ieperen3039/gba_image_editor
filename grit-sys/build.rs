use std::env;
use std::path::PathBuf;

fn main() {
    // Compile Grit's C++ sources
    cc::Build::new()
        .cpp(true)
        .include("include")
        .file("cprs.cpp")
        .file("cprs_huff.cpp")
        .file("cprs_lz.cpp")
        .file("cprs_rle.cpp")
        .file("grit_core.cpp")
        .file("grit_misc.cpp")
        .file("grit_prep.cpp")
        .file("grit_shared.cpp")
        .file("grit_xp.cpp")
        .file("logger.cpp")
        .file("pathfun.cpp")
        .compile("grit");

    // Link the static library
    println!("cargo:rustc-link-lib=static=grit");

    // Link C++ standard library (necessary for C++ code on Unix-like systems)
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // Generate bindings from Grit's C headers
    let bindings = bindgen::Builder::default()
        .header("include/grit.h")
        // Add other headers as needed
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
