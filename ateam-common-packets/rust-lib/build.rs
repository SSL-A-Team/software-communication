use std::env;
use std::path::{Path, PathBuf};

extern crate bindgen;
use bindgen::EnumVariation;

extern crate which;
use which::which;

fn is_arm_none_eabi_sysroot_valid(
    sysroot: impl AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>,
) -> bool {
    let sysroot_include = Path::new(&sysroot).join("arm-none-eabi/include");
    if !sysroot_include.exists() {
        eprintln!("the sysroot does not exist, or is malformed because it does not contain the header include folder");
        return false;
    }

    let cdefs_header = Path::new(&sysroot_include).join("sys/cdefs.h");
    if !cdefs_header.exists() {
        eprintln!("the include root appears malformed because it does not contain sys/cdefs.h");
        return false;
    }

    let types_header = Path::new(&sysroot_include).join("machine/_types.h");
    if !types_header.exists() {
        eprintln!(
            "the include root appears malformed because it does not contain machine/_types.h"
        );
        return false;
    }

    true
}

fn find_gcc_arm_embedded_sysroot() -> PathBuf {
    if let Some(user_specified_root) = env::var_os("ARM_NONE_EABI_ROOT") {
        println!(
            "user specified a embedded sysroot via $ARM_NONE_EABI_ROOT={:?}",
            user_specified_root
        );
        if is_arm_none_eabi_sysroot_valid(user_specified_root.clone()) {
            println!("OK. the specified root appears valid.");
            return PathBuf::from(user_specified_root);
        } else {
            eprintln!("ERR. User specified root appears invalid.");
        }
    }

    if let Ok(located_arm_gcc_bin) = which("arm-none-eabi-gcc") {
        println!("located arm-none-eabi-gcc on the path, attempting to find the sysroot");
        // gcc-arm-none-eabi is in gcc-arm-embedded/bin/gcc-arm-none-eabi. Two parents up is the sysroot.
        let recovered_sysroot = located_arm_gcc_bin.parent().unwrap().parent().unwrap();
        if is_arm_none_eabi_sysroot_valid(recovered_sysroot) {
            println!("OK. the recovered root appears valid.");
            return recovered_sysroot.to_path_buf();
        } else {
            eprintln!("ERR. Found root appears invalid.");
        }
    } else {
        println!("arm-none-eabi-gcc not on the path.");
    }

    panic!(
        "exhausted all options to find arm-none-eabi sysroot for bindgen. 
        Ensure arm-none-eabi-gcc is on the path and fully installed, 
        or manually set $ARM_NONE_EABI_ROOT in the environment."
    );
}

fn create_configured_builder() -> bindgen::Builder {
    let mut sysroot = find_gcc_arm_embedded_sysroot();
    sysroot.push("arm-none-eabi/include/");

    let bindings = bindgen::Builder::default()
        // Tell bindgen to use core instead of std
        // this will target embedded, so we don't have std
        .use_core()
        .ctypes_prefix("::core::ffi")
        // tell clang where to include system definitions
        .clang_arg("-target")
        .clang_arg(std::env::var("TARGET").unwrap())
        .clang_arg(format!("-I{}", sysroot.to_str().unwrap()))
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
