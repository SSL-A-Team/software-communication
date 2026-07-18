#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(const_cmp)]

use crate::bindings::{BasicControl, BodyControlMode, ExtendedTelemetry, GlobalAccelerationCommand, GlobalPositionCommand, GlobalVelocityCommand, HeadingPivotCommand, HeadingLineCommand, LocalAccelerationCommand, LocalVelocityCommand, PointPivotCommand, PointLineCommand, RadioHeader};

pub mod bindings;
pub mod radio;
pub mod proto_packets;
pub mod translation;

// TODO these are assuming the biggest packet doesn't change... const core::cmp::max is still unstable. We
// can fix this on the next major rust version update.
pub const MAX_ROBOT_RX_PACKET_SIZE: usize = core::mem::size_of::<RadioHeader>() + core::mem::size_of::<BasicControl>();
pub const MAX_ROBOT_TX_PACKET_SIZE: usize = core::mem::size_of::<RadioHeader>() + core::mem::size_of::<ExtendedTelemetry>();

pub fn is_bcm_global_position_safe(gpc: &GlobalPositionCommand) -> bool {
    gpc.global_x.is_finite() 
        && gpc.global_y.is_finite()
        && gpc.global_theta.is_finite()
        && gpc.max_linear_vel.is_finite()
        && gpc.max_angular_vel.is_finite()
        && gpc.max_linear_acc.is_finite()
        && gpc.max_angular_acc.is_finite()
}

pub fn is_bcm_global_vel_safe(gpc: &GlobalVelocityCommand) -> bool {
    gpc.global_xd.is_finite() 
        && gpc.global_yd.is_finite()
        && gpc.global_omega.is_finite()
        && gpc.max_linear_acc.is_finite()
        && gpc.max_angular_acc.is_finite()
}

pub fn is_bcm_local_vel_safe(gpc: &LocalVelocityCommand) -> bool {
    gpc.local_xd.is_finite() 
        && gpc.local_yd.is_finite()
        && gpc.local_omega.is_finite()
        && gpc.max_linear_acc.is_finite()
        && gpc.max_angular_acc.is_finite()
}

pub fn is_bcm_global_accel_safe(gpc: &GlobalAccelerationCommand) -> bool {
    gpc.global_xdd.is_finite() 
        && gpc.global_ydd.is_finite()
        && gpc.global_alpha.is_finite()
}

pub fn is_bcm_local_accel_safe(gpc: &LocalAccelerationCommand) -> bool {
    gpc.local_xdd.is_finite() 
        && gpc.local_ydd.is_finite()
        && gpc.local_alpha.is_finite()
}

pub fn is_bcm_heading_pivot_safe(gpc: &HeadingPivotCommand) -> bool {
    gpc.global_theta.is_finite()
        && gpc.max_angular_acc.is_finite()
        && gpc.max_angular_vel.is_finite()
        && gpc.orbit_radius.is_finite()
        && gpc.inset_angle.is_finite()
}

pub fn is_bcm_point_pivot_safe(gpc: &PointPivotCommand) -> bool {
    gpc.target_x.is_finite()
        && gpc.target_y.is_finite()
        && gpc.max_angular_acc.is_finite()
        && gpc.max_angular_vel.is_finite()
        && gpc.orbit_radius.is_finite()
        && gpc.inset_angle.is_finite()
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

pub fn is_basic_control_packet_safe(basic_control: &BasicControl) -> bool {
    let vision_update_safe = basic_control.vision_position_update.iter().all(|vis| vis.is_finite());
    let kicker_command_safe = basic_control.kick_vel.is_finite() && basic_control.dribbler_setpoint.is_finite();

    // SAFETY: body_control_mode is the discriminant written alongside the union;
    // each unsafe arm only reads the union field that matches the discriminant.
    let body_control_command_safe = match basic_control.body_control_mode {
        BodyControlMode::BCM_OFF => true,
        BodyControlMode::BCM_ESTOP_BRAKE => true,
        BodyControlMode::BCM_GLOBAL_POSITION => is_bcm_global_position_safe(unsafe { &basic_control.cmd.global_pos }),
        BodyControlMode::BCM_GLOBAL_VELOCITY => is_bcm_global_vel_safe(unsafe { &basic_control.cmd.global_vel }),
        BodyControlMode::BCM_LOCAL_VELOCITY => is_bcm_local_vel_safe(unsafe { &basic_control.cmd.local_vel }),
        BodyControlMode::BCM_GLOBAL_ACCEL => is_bcm_global_accel_safe(unsafe { &basic_control.cmd.global_acc }),
        BodyControlMode::BCM_LOCAL_ACCEL => is_bcm_local_accel_safe(unsafe { &basic_control.cmd.local_acc }),
        BodyControlMode::BCM_HEADING_PIVOT => is_bcm_heading_pivot_safe(unsafe { &basic_control.cmd.heading_pivot }),
        BodyControlMode::BCM_POINT_PIVOT => is_bcm_point_pivot_safe(unsafe { &basic_control.cmd.point_pivot }),
        BodyControlMode::BCM_HEADING_LINE => is_bcm_heading_line_safe(unsafe { &basic_control.cmd.heading_line }),
        BodyControlMode::BCM_POINT_LINE => is_bcm_point_line_safe(unsafe { &basic_control.cmd.point_line }),
        _ => false
    };

    return vision_update_safe && kicker_command_safe && body_control_command_safe;
}