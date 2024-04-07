extern crate bindgen;

use std::{env, fs};
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

fn main() {

    let wrapper_file_path = format!("{}/{}", env::var("OUT_DIR").expect("env var should be set"), "wrapper.h");

    // Open the wrapper file
    let mut wrapper_file = File::create(&wrapper_file_path).unwrap();

    // List the relevant header files to build bindings for.
    let header_files_apache: Vec<String> = fs::read_dir("/usr/include/apache2").unwrap().map(|r| r.unwrap().file_name().to_str().unwrap().into()).collect();
    let header_files_apr: Vec<String> = fs::read_dir("/usr/include/apr-1.0").unwrap().map(|r| r.unwrap().file_name().to_str().unwrap().into()).collect();

    // Join the lists of header files.
    let mut header_files: Vec<String> = [header_files_apache, header_files_apr].concat();

    // TODO: fix dependency
    // Remove specific header files because of known issues.
    header_files.retain(|header_file| !header_file.eq("mod_xml2enc.h"));

    // Sort the list of header files.
    header_files.sort();

    // Write include statement into wrapper file.
    for header_file in header_files {
        writeln!(&mut wrapper_file, "#include \"{}\"", header_file).unwrap();
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes.
    println!("cargo:rerun-if-changed={}", &wrapper_file_path);

    // The bindgen::Builder
    // Derived from https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    let bindings = bindgen::Builder::default()
        // Use signed integer per default for magic numbers defined as macro
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        // 128-bit integers do not have a stable ABI, so we need to remove functions that make use of them
        .blocklist_function("qecvt")
        .blocklist_function("qecvt_r")
        .blocklist_function("qfcvt")
        .blocklist_function("qfcvt_r")
        .blocklist_function("qgcvt")
        .blocklist_function("strfromf64x")
        .blocklist_function("strfroml")
        .blocklist_function("strtof64x")
        .blocklist_function("strtold")
        // The input header we would like to generate bindings for.
        .header(&wrapper_file_path)
        // Add the includes for C header files.
        // Derived from https://httpd.apache.org/docs/2.4/developer/modguide.html
        // $ apxs -a -c mod_example.c
        .clang_arg("-I/usr/include/apache2")
        .clang_arg("-I/usr/include/apr-1.0")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
