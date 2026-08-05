use crate::bitfields::{KickerCommandFlags, KickerStatusFlags};
use crate::packets::stspin_current::CcmTelemetry;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum KickRequest {
    #[default]
    Disable      = 0,
    Arm          = 1,
    KickNow      = 2,
    KickTouch    = 3,
    KickCaptured = 4,
    ChipNow      = 5,
    ChipTouch    = 6,
    ChipCaptured = 7,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DribblerCommand {
    #[default]
    Disable     = 0,
    HardReceive = 1,
    SoftReceive = 2,
    Dribble     = 3,
    Velocity    = 4,
    Current     = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct KickerControl {
    pub flags:          KickerCommandFlags,
    pub dribbler_mode:  DribblerCommand,
    pub kick_request:   KickRequest,
    pub kick_speed:     f32,
    pub drib_setpoint:  f32,
}
const _: () = assert!(core::mem::size_of::<KickerControl>() == 12);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct KickerTelemetry {
    pub status:             KickerStatusFlags,
    pub charge_pct:         u16,
    pub rail_voltage:       f32,
    pub battery_voltage:    f32,
    pub kicker_image_hash:  [u8; 4],
    pub dribbler_motor:     CcmTelemetry,
}
const _: () = assert!(core::mem::size_of::<KickerTelemetry>() == 76);
