use crate::bitfields::BasicTelemetryErrors;
use crate::packets::maneuvers::*;

/// Body control telemetry discriminated union.
/// Layout: tag (1B) + padding (3B) + body (4B) = 8B.
#[repr(C, u8)]
#[derive(Clone, Copy, Debug, Default)]
pub enum BodyControlTelemetry {
    #[default]
    Off             = 0,
    EstopBrake      = 1,
    GlobalPosition(GlobalPositionTelemetry)         = 10,
    GlobalVelocity(GlobalVelocityTelemetry)         = 11,
    LocalVelocity(LocalVelocityTelemetry)           = 12,
    GlobalAcceleration(GlobalAccelerationTelemetry) = 13,
    LocalAcceleration(LocalAccelerationTelemetry)   = 14,
    HeadingPivot(HeadingPivotTelemetry)             = 20,
    PointPivot(PointPivotTelemetry)                 = 21,
    HeadingLine(HeadingLineTelemetry)               = 30,
    PointLine(PointLineTelemetry)                   = 31,
}
const _: () = assert!(core::mem::size_of::<BodyControlTelemetry>() == 8);

/// 28 → 32 bytes. `body_control_mode` removed; now the discriminant of
/// `control_telem`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BasicTelemetry {
    pub transmission_sequence_number:   u8,
    pub control_data_sequence_number:   u8,
    pub _reserved:                      [u8; 2],
    pub errors:                         BasicTelemetryErrors,
    pub battery_percent:                u16,
    pub kicker_charge_percent:          u16,
    /// Self-discriminating telemetry; match on variants directly.
    pub control_telem:                  BodyControlTelemetry,
    /// Quantized KF position estimate [x mm, y mm, theta mrad].
    pub kf_body_pos_estimate:           [i16; 3],
    /// Quantized KF velocity estimate [vx mm/s, vy mm/s, omega mrad/s].
    pub kf_body_vel_estimate:           [i16; 3],
}
const _: () = assert!(core::mem::size_of::<BasicTelemetry>() == 32);
