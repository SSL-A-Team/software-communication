use crate::bitfields::{CcmMotionCommandFlags, CcmTelemetryStatus};

// ─── Shared motion control type ───────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CcmMotionControlType {
    #[default]
    MotorOff        = 0,
    DutyOpenloop    = 1,
    VoltageOpenloop = 2,
    Current         = 3,
    Velocity        = 4,
    VelocityCurrent = 5,
}

// ─── CcmMotionCommand (used on motor SPI bus; 12B packed) ────────────────────

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CcmMotionCommand {
    pub flags:                  CcmMotionCommandFlags,
    pub motion_control_type:    CcmMotionControlType,
    pub _reserved:              [u8; 1],
    pub current_setpoint_ma:    i16,
    pub setpoint:               f32,
}
const _: () = assert!(core::mem::size_of::<CcmMotionCommand>() == 12);

// ─── CcmTelemetry (radio wire; in ExtendedTelemetry) ─────────────────────────

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CcmCurrentTelemetry {
    pub bus_voltage_mv:         u16,
    pub motor_voltage_cmd_mv:   u16,
    pub current_setpoint_ma:    i16,
    pub hall_vel_est_drads:     i16,
    pub current_samples_ma:     [u16; 20],
}
const _: () = assert!(core::mem::size_of::<CcmCurrentTelemetry>() == 48);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CcmVelocityTelemetry {
    pub vel_setpoint_rads:  f32,
    pub wheel_vel_rads:     f32,
}
const _: () = assert!(core::mem::size_of::<CcmVelocityTelemetry>() == 8);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CcmTelemetry {
    pub status:             CcmTelemetryStatus,
    pub motion_control_type: CcmMotionControlType,
    pub gain_stage_index:   u8,
    pub current_telemetry:  CcmCurrentTelemetry,
    pub velocity_telemetry: CcmVelocityTelemetry,
}
const _: () = assert!(core::mem::size_of::<CcmTelemetry>() == 60);

// ─── CcmParameterPacket (internal SPI parameter protocol) ────────────────────

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcmCommandType {
    Params = 0x20,
    Motion = 0x21,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CcmParameter {
    #[default]
    CurrentPiGainP          = 0,
    CurrentPiGainI          = 1,
    CurrentPiErrIlim        = 2,
    CurrentPiAntijitter     = 3,
    FirmwareImageHash       = 100,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CcmParameterOperation {
    #[default]
    Read  = 0,
    Write = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CcmParameterDirection {
    #[default]
    Command = 0,
    Reply   = 1,
}

/// 4-byte union value — stored as raw bytes; caller transmutes as needed.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CcmParameterValue(pub [u8; 4]);

impl CcmParameterValue {
    pub fn as_u32(self) -> u32 { u32::from_le_bytes(self.0) }
    pub fn as_i32(self) -> i32 { i32::from_le_bytes(self.0) }
    pub fn as_f32(self) -> f32 { f32::from_le_bytes(self.0) }
    pub fn from_u32(v: u32) -> Self { Self(v.to_le_bytes()) }
    pub fn from_i32(v: i32) -> Self { Self(v.to_le_bytes()) }
    pub fn from_f32(v: f32) -> Self { Self(v.to_le_bytes()) }
}
const _: () = assert!(core::mem::size_of::<CcmParameterValue>() == 4);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CcmParameterPacket {
    pub parameter:              CcmParameter,
    pub parameter_operation:    CcmParameterOperation,
    pub parameter_direction:    CcmParameterDirection,
    pub _reserved:              [u8; 1],
    pub value:                  CcmParameterValue,
}
const _: () = assert!(core::mem::size_of::<CcmParameterPacket>() == 8);

/// CcmCommand data discriminated by `CcmCommandType`. Because the tag (`type`)
/// and the data body are not adjacent in the original SPI protocol (crc32 sits
/// between them), we represent this as a plain struct with an unsafe union
/// rather than `#[repr(C, u8)]`.
#[repr(C)]
#[derive(Clone, Copy)]
pub union CcmCommandData {
    pub param:  CcmParameterPacket,
    pub motion: CcmMotionCommand,
}
const _: () = assert!(core::mem::size_of::<CcmCommandData>() == 12);

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct CcmCommand {
    pub cmd_type:   CcmCommandType,
    pub _reserved:  [u8; 3],
    pub crc32:      u32,
    pub data:       CcmCommandData,
}
const _: () = assert!(core::mem::size_of::<CcmCommand>() == 20);

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcmResponseType {
    Params = 0x80,
    Telem  = 0x81,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union CcmResponseData {
    pub params: CcmParameterPacket,
    pub motion: CcmTelemetry,
}
const _: () = assert!(core::mem::size_of::<CcmResponseData>() == 60);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CcmResponse {
    pub resp_type:  CcmResponseType,
    pub seq_num:    u8,
    pub _reserved:  [u8; 2],
    pub timestamp:  u32,
    pub data:       CcmResponseData,
}
const _: () = assert!(core::mem::size_of::<CcmResponse>() == 68);

impl Default for CcmResponse {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
