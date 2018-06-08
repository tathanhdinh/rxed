extern crate bindgen;

fn main() {
    println!("cargo:rustc-link-lib={}={}", "static", "xed");
    // println!("cargo:rustc-link-lib={}={}", "static", "xed-ild");
    println!("cargo:rustc-link-search={}={}", "native", "xed-c");
    // try with: https://github.com/rust-lang/rust/issues/27142
    println!("cargo:rustc-codegen={}={}", "relocation-model", "static");

    let bindings = bindgen::Builder::default()
        .header("xed-c/include/xed/xed-interface.h")
        .prepend_enum_name(false)
        .generate()
        .expect("Could not generate bindings for xed!");

    let outpath = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings.write_to_file(outpath.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}