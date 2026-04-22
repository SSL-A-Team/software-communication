#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(const_cmp)]

use crate::bindings::{BasicControl, BodyControlMode, ExtendedTelemetry, GlobalAccelerationCommand, GlobalPositionCommand, GlobalVelocityCommand, LocalAccelerationCommand, LocalVelocityCommand, RadioHeader};

pub mod bindings;
pub mod radio;

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
    gpc.local_xdd.is_finite() 
        && gpc.local_ydd.is_finite()
        && gpc.local_alpha.is_finite()
}

pub fn is_bcm_local_accel_safe(gpc: &LocalAccelerationCommand) -> bool {
    gpc.local_xdd.is_finite() 
        && gpc.local_ydd.is_finite()
        && gpc.local_alpha.is_finite()
}

pub fn is_basic_control_packet_safe(basic_control: &BasicControl) -> bool {
    let vision_update_safe = basic_control.vision_position_update.iter().all(|vis| vis.is_finite());
    let kicker_command_safe = basic_control.kick_vel.is_finite() && basic_control.dribbler_speed.is_finite();

    let body_control_command_safe = match basic_control.body_control_mode {
        BodyControlMode::BCM_OFF => true,
        BodyControlMode::BCM_GLOBAL_POSITION => is_bcm_global_position_safe(unsafe { &basic_control.cmd.global_pos }),
        BodyControlMode::BCM_GLOBAL_VELOCITY => is_bcm_global_vel_safe(unsafe { &basic_control.cmd.global_vel }),
        BodyControlMode::BCM_LOCAL_VELOCITY => is_bcm_local_vel_safe(unsafe { &basic_control.cmd.local_vel }),
        BodyControlMode::BCM_GLOBAL_ACCEL => is_bcm_global_accel_safe(unsafe { &basic_control.cmd.global_acc }),
        BodyControlMode::BCM_LOCAL_ACCEL => is_bcm_local_accel_safe(unsafe { &basic_control.cmd.local_acc }),
        _ => false
    };

    return vision_update_safe && kicker_command_safe && body_control_command_safe;
}