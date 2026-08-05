use crate::packets::body_control::BodyControlExtendedTelemetry;
use crate::packets::kicker::KickerTelemetry;
use crate::packets::power::PowerTelemetry;
use crate::packets::stspin_current::CcmTelemetry;

/// 556 → 560 bytes (BodyControlExtendedTelemetry grows 4B).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExtendedTelemetry {
    pub timestamp_us_lo:            u32,
    pub timestamp_us_hi:            u32,
    pub power_status:               PowerTelemetry,
    pub front_left_motor:           CcmTelemetry,
    pub back_left_motor:            CcmTelemetry,
    pub back_right_motor:           CcmTelemetry,
    pub front_right_motor:          CcmTelemetry,
    pub body_control_telemetry:     BodyControlExtendedTelemetry,
    pub kicker_status:              KickerTelemetry,
}
const _: () = assert!(core::mem::size_of::<ExtendedTelemetry>() == 560);
