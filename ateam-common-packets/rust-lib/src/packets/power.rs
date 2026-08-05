use crate::bitfields::{BatteryStatus, PowerCommandFlags, PowerStatus};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BatteryInfo {
    pub status:         BatteryStatus,
    pub battery_mv:     u16,
    pub cell1_mv:       u16,
    pub cell2_mv:       u16,
    pub cell3_mv:       u16,
    pub cell4_mv:       u16,
    pub cell5_mv:       u16,
    pub cell6_mv:       u16,
    pub battery_pct:    u8,
    pub cell1_pct:      u8,
    pub cell2_pct:      u8,
    pub cell3_pct:      u8,
    pub cell4_pct:      u8,
    pub cell5_pct:      u8,
    pub cell6_pct:      u8,
    pub _reserved:      u8,
}
const _: () = assert!(core::mem::size_of::<BatteryInfo>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PowerTelemetry {
    pub status:         PowerStatus,
    pub battery_info:   BatteryInfo,
}
const _: () = assert!(core::mem::size_of::<PowerTelemetry>() == 28);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PowerCommand {
    pub flags: PowerCommandFlags,
}
const _: () = assert!(core::mem::size_of::<PowerCommand>() == 4);
