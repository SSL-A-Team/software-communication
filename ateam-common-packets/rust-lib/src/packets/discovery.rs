use crate::bitfields::DiscoveryFlags;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeamColor {
    Yellow = 0,
    Blue   = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HelloRequest {
    pub robot_id:           u8,
    pub color:              TeamColor,
    pub flags:              DiscoveryFlags,
    pub _reserved:          [u8; 1],
    pub coms_hash:          [u8; 4],
    pub controls_hash:      [u8; 4],
    pub firmware_hash:      [u8; 4],
}
const _: () = assert!(core::mem::size_of::<HelloRequest>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HelloResponse {
    pub ipv4:   [u8; 4],
    pub port:   u16,
}
const _: () = assert!(core::mem::size_of::<HelloResponse>() == 6);
