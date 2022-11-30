#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(core_ffi_c)]

pub mod stspin_packets {
	include!("stspin_bindings.rs");
}

