use crate::bitfields::BodyControlExtTelemetryFlags;
use crate::packets::maneuvers::*;

/// Extended body control maneuver telemetry discriminated union.
/// Layout: tag (1B) + padding (3B) + body (56B) = 60B.
#[repr(C, u8)]
#[derive(Clone, Copy, Debug, Default)]
pub enum BodyControlManeuverExtendedTelemetry {
    #[default]
    Off             = 0,
    EstopBrake      = 1,
    GlobalPosition(ExtendedGlobalPositionTelemetry)         = 10,
    GlobalVelocity(ExtendedGlobalVelocityTelemetry)         = 11,
    LocalVelocity(ExtendedLocalVelocityTelemetry)           = 12,
    GlobalAcceleration(ExtendedGlobalAccelerationTelemetry) = 13,
    LocalAcceleration(ExtendedLocalAccelerationTelemetry)   = 14,
    HeadingPivot(ExtendedHeadingPivotTelemetry)             = 20,
    PointPivot(ExtendedPointPivotTelemetry)                 = 21,
    HeadingLine(ExtendedHeadingLineTelemetry)               = 30,
    PointLine(ExtendedPointLineTelemetry)                   = 31,
}
const _: () = assert!(core::mem::size_of::<BodyControlManeuverExtendedTelemetry>() == 60);

/// 204 → 208 bytes (maneuver field grows 4B from discriminant overhead).
/// All float array fields are `[f32; 3]` — do not substitute `Vector3<f32>`;
/// nalgebra does not guarantee transparent layout over `[f32; 3]`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BodyControlExtendedTelemetry {
    pub flags:                      BodyControlExtTelemetryFlags,
    pub _reserved:                  [u8; 3],
    pub maneuver:                   BodyControlManeuverExtendedTelemetry,
    pub imu_gyro:                   [f32; 3],
    pub imu_accel:                  [f32; 3],
    pub vision_pose:                [f32; 3],
    pub body_traj_pos:              [f32; 3],
    pub body_traj_vel:              [f32; 3],
    pub kf_body_pos_prediction:     [f32; 3],
    pub kf_body_vel_prediction:     [f32; 3],
    pub kf_body_pos_estimate:       [f32; 3],
    pub kf_body_vel_estimate:       [f32; 3],
    pub body_vel_u:                 [f32; 3],
    pub body_accel_u:               [f32; 3],
    pub body_accel_u_fric_comp:     [f32; 3],
}
const _: () = assert!(core::mem::size_of::<BodyControlExtendedTelemetry>() == 208);
