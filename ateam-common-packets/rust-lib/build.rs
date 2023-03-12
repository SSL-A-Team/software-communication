extern crate bindgen;
use bindgen::EnumVariation;
use std::path::Path;

fn main() {
    // Tell cargo to invalidate the built crate whenever the sources change
    println!("cargo:rerun-if-changed=../include/");

    let bindings_stspin = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("../include/stspin.h")
        .allowlist_file(".*/stspin.h")
        // Tell bindgen to use core instead of std
        .use_core()
        .ctypes_prefix("::core::ffi")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Put enum consts in module
        .default_enum_style(EnumVariation::ModuleConsts)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate STSPIN bindings");

    let bindings_radio = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("../include/basic_control.h")
        .header("../include/basic_telemetry.h")
        .header("../include/hello_data.h")
        .header("../include/radio.h")
        .allowlist_file(".*/basic_control.h")
        .allowlist_file(".*/basic_telemetry.h")
        .allowlist_file(".*/hello_data.h")
        .allowlist_file(".*/radio.h")
        .generate_comments(true)
        // Tell bindgen to use core instead of std
        .use_core()
        .ctypes_prefix("::core::ffi")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Put enum consts in module
        .default_enum_style(EnumVariation::ModuleConsts)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate Radio bindings");

    // Write the bindings to the lib source dir
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = "src";
    bindings_stspin
        .write_to_file(Path::new(&out_dir).join("bindings_stspin.rs"))
        .expect("Couldn't write STSPIN packet bindings!");
    bindings_radio
        .write_to_file(Path::new(&out_dir).join("bindings_radio.rs"))
        .expect("Couldn't write Radio packet bindings!");
}
