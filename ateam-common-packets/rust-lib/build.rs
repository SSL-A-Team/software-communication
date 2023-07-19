extern crate bindgen;
use bindgen::EnumVariation;
use std::env;
use std::path::{Path, PathBuf};

fn find_gcc_arm_embedded_incroot() -> PathBuf {
    let arm_none_eabi_root = env::var_os("ARM_NONE_EABI_ROOT").expect(
        "the ABI definition root was not set.
        Are you in the Nix environment?
        Set \"$ARM_NONE_EABI_ROOT=<root of arm-none-eabi-gcc>\"",
    );

    let sysroot_include = Path::new(&arm_none_eabi_root).join("arm-none-eabi/include");
    if !sysroot_include.exists() {
        panic!("the sysroot does not exist, or is malformed because it does not contain the header include folder");
    }

    let cdefs_header = Path::new(&sysroot_include).join("sys/cdefs.h");
    if !cdefs_header.exists() {
        panic!("the include root appears malformed because it does not contain sys/cdefs.h");
    }

    let types_header = Path::new(&sysroot_include).join("machine/_types.h");
    if !types_header.exists() {
        panic!("the include root appears malformed because it does not contain machine/_types.h");
    }

    sysroot_include.to_path_buf()
}

fn create_configured_builder() -> bindgen::Builder {
    let incroot = find_gcc_arm_embedded_incroot();

    let bindings = bindgen::Builder::default()
        // Tell bindgen to use core instead of std
        // this will target embedded, so we don't have std
        .use_core()
        .ctypes_prefix("::core::ffi")
        // tell clang where to include system definitions
        .clang_arg("-target")
        .clang_arg(std::env::var("TARGET").unwrap())
        .clang_arg(format!("-I{}", incroot.to_str().unwrap()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Put enum consts in module
        .default_enum_style(EnumVariation::ModuleConsts)
        .generate_comments(true);

    bindings
}

fn main() {
    // Tell cargo to invalidate the built crate whenever the sources change
    println!("cargo:rerun-if-changed=../include/");

    let bindings_stspin = create_configured_builder()
        // The input header we would like to generate
        // bindings for.
        .header("../include/stspin.h")
        .allowlist_file(".*/stspin.h")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate STSPIN bindings");

    let bindings_kicker = create_configured_builder()
        // The input header we would like to generate
        // bindings for.
        .header("../include/kicker.h")
        .allowlist_file(".*/kicker.h")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate Kicker bindings");

    let bindings_radio = create_configured_builder()
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
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate Radio bindings");

    // Write the bindings to the lib source dir
    let out_dir = "src";
    bindings_stspin
        .write_to_file(Path::new(&out_dir).join("bindings_stspin.rs"))
        .expect("Couldn't write STSPIN packet bindings!");
    bindings_kicker
        .write_to_file(Path::new(&out_dir).join("bindings_kicker.rs"))
        .expect("Couldn't write Kicker packet bindings!");
    bindings_radio
        .write_to_file(Path::new(&out_dir).join("bindings_radio.rs"))
        .expect("Couldn't write Radio packet bindings!");
}
