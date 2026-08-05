#![no_std]

pub mod bitfields;
pub mod packets;
pub mod radio;

// Expose the most-used types at the crate root for convenience.
pub use packets::basic_control::{BasicControl, BodyControlCommand};
pub use packets::basic_telemetry::{BasicTelemetry, BodyControlTelemetry};
pub use packets::body_control::{BodyControlExtendedTelemetry, BodyControlManeuverExtendedTelemetry};
pub use packets::discovery::{HelloRequest, HelloResponse, TeamColor};
pub use packets::error_telemetry::ErrorTelemetry;
pub use packets::extended_telemetry::ExtendedTelemetry;
pub use packets::kicker::{DribblerCommand, KickRequest, KickerControl, KickerTelemetry};
pub use packets::maneuvers::*;
pub use packets::power::{BatteryInfo, PowerCommand, PowerTelemetry};
pub use packets::radio::{RadioData, RadioHeader, RadioPacket};
pub use packets::robot_parameters::{ParameterCommand, ParameterData, ParameterName};
pub use packets::robot_parameters::ParameterCommandCode;
pub use packets::stspin_current::{
    CcmCommand, CcmCommandData, CcmCommandType,
    CcmMotionCommand, CcmMotionControlType, CcmTelemetry, CcmCurrentTelemetry,
    CcmVelocityTelemetry,
    CcmParameter, CcmParameterDirection, CcmParameterOperation, CcmParameterPacket,
    CcmParameterValue, CcmResponse, CcmResponseData, CcmResponseType,
};
pub use bitfields::CcmMotionCommandFlags;

pub const MAX_ROBOT_RX_PACKET_SIZE: usize =
    core::mem::size_of::<RadioHeader>() + core::mem::size_of::<BasicControl>();
pub const MAX_ROBOT_TX_PACKET_SIZE: usize =
    core::mem::size_of::<RadioHeader>() + core::mem::size_of::<ExtendedTelemetry>();

// ─── Packet validation ────────────────────────────────────────────────────────

pub fn is_bcm_global_position_safe(c: &GlobalPositionCommand) -> bool {
    c.global_x.is_finite()
        && c.global_y.is_finite()
        && c.global_theta.is_finite()
        && c.max_linear_vel.is_finite()
        && c.max_angular_vel.is_finite()
        && c.max_linear_acc.is_finite()
        && c.max_angular_acc.is_finite()
}

pub fn is_bcm_global_vel_safe(c: &GlobalVelocityCommand) -> bool {
    c.global_xd.is_finite()
        && c.global_yd.is_finite()
        && c.global_omega.is_finite()
        && c.max_linear_acc.is_finite()
        && c.max_angular_acc.is_finite()
}

pub fn is_bcm_local_vel_safe(c: &LocalVelocityCommand) -> bool {
    c.local_xd.is_finite()
        && c.local_yd.is_finite()
        && c.local_omega.is_finite()
        && c.max_linear_acc.is_finite()
        && c.max_angular_acc.is_finite()
}

pub fn is_bcm_global_accel_safe(c: &GlobalAccelerationCommand) -> bool {
    c.global_xdd.is_finite() && c.global_ydd.is_finite() && c.global_alpha.is_finite()
}

pub fn is_bcm_local_accel_safe(c: &LocalAccelerationCommand) -> bool {
    c.local_xdd.is_finite() && c.local_ydd.is_finite() && c.local_alpha.is_finite()
}

pub fn is_bcm_heading_pivot_safe(c: &HeadingPivotCommand) -> bool {
    c.global_theta.is_finite()
        && c.max_angular_vel.is_finite()
        && c.max_angular_acc.is_finite()
        && c.orbit_radius.is_finite()
        && c.inset_angle.is_finite()
}

pub fn is_bcm_point_pivot_safe(c: &PointPivotCommand) -> bool {
    c.target_x.is_finite()
        && c.target_y.is_finite()
        && c.max_angular_vel.is_finite()
        && c.max_angular_acc.is_finite()
        && c.orbit_radius.is_finite()
        && c.inset_angle.is_finite()
}

pub fn is_bcm_heading_line_safe(c: &HeadingLineCommand) -> bool {
    c.start_x.is_finite()
        && c.start_y.is_finite()
        && c.dir_x.is_finite()
        && c.dir_y.is_finite()
        && c.line_velocity.is_finite()
        && c.global_theta.is_finite()
        && c.max_vel_colinear.is_finite()
        && c.max_vel_perp.is_finite()
        && c.max_vel_angular.is_finite()
        && c.max_accel_colinear.is_finite()
        && c.max_accel_perp.is_finite()
        && c.max_accel_angular.is_finite()
        && c.colinear_start_thresh.is_finite()
}

pub fn is_bcm_point_line_safe(c: &PointLineCommand) -> bool {
    c.start_x.is_finite()
        && c.start_y.is_finite()
        && c.dir_x.is_finite()
        && c.dir_y.is_finite()
        && c.line_velocity.is_finite()
        && c.target_x.is_finite()
        && c.target_y.is_finite()
        && c.max_vel_colinear.is_finite()
        && c.max_vel_perp.is_finite()
        && c.max_vel_angular.is_finite()
        && c.max_accel_colinear.is_finite()
        && c.max_accel_perp.is_finite()
        && c.max_accel_angular.is_finite()
        && c.colinear_start_thresh.is_finite()
}

pub fn is_basic_control_packet_safe(bc: &BasicControl) -> bool {
    let vision_ok = bc.vision_position_update.iter().all(|v| v.is_finite());
    let kicker_ok = bc.kick_vel.is_finite() && bc.dribbler_setpoint.is_finite();
    let cmd_ok = match &bc.cmd {
        BodyControlCommand::Off | BodyControlCommand::EstopBrake => true,
        BodyControlCommand::GlobalPosition(c)    => is_bcm_global_position_safe(c),
        BodyControlCommand::GlobalVelocity(c)    => is_bcm_global_vel_safe(c),
        BodyControlCommand::LocalVelocity(c)     => is_bcm_local_vel_safe(c),
        BodyControlCommand::GlobalAcceleration(c) => is_bcm_global_accel_safe(c),
        BodyControlCommand::LocalAcceleration(c) => is_bcm_local_accel_safe(c),
        BodyControlCommand::HeadingPivot(c)      => is_bcm_heading_pivot_safe(c),
        BodyControlCommand::PointPivot(c)        => is_bcm_point_pivot_safe(c),
        BodyControlCommand::HeadingLine(c)       => is_bcm_heading_line_safe(c),
        BodyControlCommand::PointLine(c)         => is_bcm_point_line_safe(c),
    };
    vision_ok && kicker_ok && cmd_ok
}
