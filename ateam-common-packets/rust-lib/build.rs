extern crate bindgen;

use std::fs;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the sources change
    println!("cargo:rerun-if-changed=../include/");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("::core::ffi")
        // The input header we would like to generate
        // bindings for.
        .header("../include/basic_control.h")
        .header("../include/basic_telemetry.h")
        .header("../include/hello_data.h")
        .header("../include/radio.h")
        .header("../include/stspin.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the lib source dir
    let src_dir = "src";
    fs::create_dir_all(src_dir).expect("Failed to create src dir.");

    let out_path = PathBuf::from(src_dir);
    bindings.write_to_file(out_path.join("stspin_bindings.rs"))
        .expect("Couldn't write STSPIN packet bindings!");
}
