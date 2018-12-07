extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link libmoi
    println!("cargo:rustc-link-lib=mobi");

    // Generate the bindings
    let bindings = bindgen::Builder::default()
        .header("src/headers/wrapper.h")
        .generate()
        .expect("Bindings could not be generated");

    // Write bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Bindings could not be wirtten");

}
