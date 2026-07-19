// Translation layer between C-layout packed structs (bindgen) and
// micropb-generated proto structs. Used to convert between the SPI wire format
// and the radio proto encoding.
//
// Two macros handle the straightforward field-copy cases.  Complex cases
// (bitfields, submessage setters, array↔Vec) are written as manual impls below.
//
// Macro hygiene note: $src and $out are user-provided idents so that the
// field expressions and statement blocks share the same hygiene context.

use crate::bindings as c;
use crate::proto_packets::ateam_ as proto;

// Compile-time checks that proto enum values match C constants.
// These are plain numeric casts — a mismatch silently sends wrong commands over radio.
const _: () = {
    assert!(proto::KickRequest::KrArm.0          == c::KickRequest::KR_ARM           as i32);
    assert!(proto::KickRequest::KrDisable.0      == c::KickRequest::KR_DISABLE       as i32);
    assert!(proto::KickRequest::KrKickNow.0      == c::KickRequest::KR_KICK_NOW      as i32);
    assert!(proto::KickRequest::KrKickTouch.0    == c::KickRequest::KR_KICK_TOUCH    as i32);
    assert!(proto::KickRequest::KrKickCaptured.0 == c::KickRequest::KR_KICK_CAPTURED as i32);
    assert!(proto::KickRequest::KrChipNow.0      == c::KickRequest::KR_CHIP_NOW      as i32);
    assert!(proto::KickRequest::KrChipTouch.0    == c::KickRequest::KR_CHIP_TOUCH    as i32);
    assert!(proto::KickRequest::KrChipCaptured.0 == c::KickRequest::KR_CHIP_CAPTURED as i32);

    assert!(proto::DribblerMode::DmDisable.0     == c::DribblerCommand::DC_DISABLE      as i32);
    assert!(proto::DribblerMode::DmHardReceive.0 == c::DribblerCommand::DC_HARD_RECEIVE as i32);
    assert!(proto::DribblerMode::DmSoftReceive.0 == c::DribblerCommand::DC_SOFT_RECEIVE as i32);
    assert!(proto::DribblerMode::DmDribble.0     == c::DribblerCommand::DC_DRIBBLE      as i32);
    assert!(proto::DribblerMode::DmVelocity.0    == c::DribblerCommand::DC_VELOCITY     as i32);
    assert!(proto::DribblerMode::DmCurrent.0     == c::DribblerCommand::DC_CURRENT      as i32);
};

// ---------------------------------------------------------------------------
// Macros
// ---------------------------------------------------------------------------

/// `impl From<&$c_type> for $p_type` via struct literal.
/// Works only for proto types with NO submessage fields (no _has).
/// $src is the parameter name — use the same ident in field expressions.
macro_rules! impl_c_to_proto {
    ($c_type:path => $p_type:path, $src:ident, { $($f:ident: $e:expr),* $(,)? }) => {
        impl From<&$c_type> for $p_type {
            fn from($src: &$c_type) -> Self {
                Self { $($f: $e,)* ..Default::default() }
            }
        }
    };
}

/// `impl From<&$p_type> for $c_type` via default + sequential field assignments.
/// $src = source param name; $out = mutable output var name.
/// Use the same idents in field expressions.
macro_rules! impl_proto_to_c {
    ($p_type:path => $c_type:path, $src:ident, $out:ident, { $($f:ident: $e:expr),* $(,)? }) => {
        impl From<&$p_type> for $c_type {
            fn from($src: &$p_type) -> Self {
                let mut $out = <$c_type>::default();
                $($out.$f = $e;)*
                $out
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Maneuver commands — all-float structs, direct copy both directions.
// Needed for BodyControlExtendedTelemetry maneuver echo translation.
// ---------------------------------------------------------------------------

impl_c_to_proto!(c::GlobalPositionCommand => proto::GlobalPositionCommand, s, {
    global_x: s.global_x, global_y: s.global_y, global_theta: s.global_theta,
    max_linear_vel: s.max_linear_vel, max_angular_vel: s.max_angular_vel,
    max_linear_acc: s.max_linear_acc, max_angular_acc: s.max_angular_acc,
});
impl_proto_to_c!(proto::GlobalPositionCommand => c::GlobalPositionCommand, s, r, {
    global_x: s.global_x, global_y: s.global_y, global_theta: s.global_theta,
    max_linear_vel: s.max_linear_vel, max_angular_vel: s.max_angular_vel,
    max_linear_acc: s.max_linear_acc, max_angular_acc: s.max_angular_acc,
});

impl_c_to_proto!(c::GlobalVelocityCommand => proto::GlobalVelocityCommand, s, {
    global_xd: s.global_xd, global_yd: s.global_yd, global_omega: s.global_omega,
    max_linear_acc: s.max_linear_acc, max_angular_acc: s.max_angular_acc,
});
impl_proto_to_c!(proto::GlobalVelocityCommand => c::GlobalVelocityCommand, s, r, {
    global_xd: s.global_xd, global_yd: s.global_yd, global_omega: s.global_omega,
    max_linear_acc: s.max_linear_acc, max_angular_acc: s.max_angular_acc,
});

impl_c_to_proto!(c::LocalVelocityCommand => proto::LocalVelocityCommand, s, {
    local_xd: s.local_xd, local_yd: s.local_yd, local_omega: s.local_omega,
    max_linear_acc: s.max_linear_acc, max_angular_acc: s.max_angular_acc,
});
impl_proto_to_c!(proto::LocalVelocityCommand => c::LocalVelocityCommand, s, r, {
    local_xd: s.local_xd, local_yd: s.local_yd, local_omega: s.local_omega,
    max_linear_acc: s.max_linear_acc, max_angular_acc: s.max_angular_acc,
});

impl_c_to_proto!(c::GlobalAccelerationCommand => proto::GlobalAccelerationCommand, s, {
    global_xdd: s.global_xdd, global_ydd: s.global_ydd, global_alpha: s.global_alpha,
});
impl_proto_to_c!(proto::GlobalAccelerationCommand => c::GlobalAccelerationCommand, s, r, {
    global_xdd: s.global_xdd, global_ydd: s.global_ydd, global_alpha: s.global_alpha,
});

impl_c_to_proto!(c::LocalAccelerationCommand => proto::LocalAccelerationCommand, s, {
    local_xdd: s.local_xdd, local_ydd: s.local_ydd, local_alpha: s.local_alpha,
});
impl_proto_to_c!(proto::LocalAccelerationCommand => c::LocalAccelerationCommand, s, r, {
    local_xdd: s.local_xdd, local_ydd: s.local_ydd, local_alpha: s.local_alpha,
});

impl_c_to_proto!(c::HeadingPivotCommand => proto::HeadingPivotCommand, s, {
    global_theta: s.global_theta, max_angular_vel: s.max_angular_vel,
    max_angular_acc: s.max_angular_acc, orbit_radius: s.orbit_radius,
    inset_angle: s.inset_angle, direction: s.direction as u32,
    compute_inset_angle: s.compute_inset_angle != 0,
});
impl_proto_to_c!(proto::HeadingPivotCommand => c::HeadingPivotCommand, s, r, {
    global_theta: s.global_theta, max_angular_vel: s.max_angular_vel,
    max_angular_acc: s.max_angular_acc, orbit_radius: s.orbit_radius,
    inset_angle: s.inset_angle, direction: s.direction as c::PivotDirection::Type,
    compute_inset_angle: s.compute_inset_angle as u8,
});

impl_c_to_proto!(c::PointPivotCommand => proto::PointPivotCommand, s, {
    target_x: s.target_x, target_y: s.target_y,
    max_angular_vel: s.max_angular_vel, max_angular_acc: s.max_angular_acc,
    orbit_radius: s.orbit_radius, inset_angle: s.inset_angle,
    direction: s.direction as u32, compute_inset_angle: s.compute_inset_angle != 0,
});
impl_proto_to_c!(proto::PointPivotCommand => c::PointPivotCommand, s, r, {
    target_x: s.target_x, target_y: s.target_y,
    max_angular_vel: s.max_angular_vel, max_angular_acc: s.max_angular_acc,
    orbit_radius: s.orbit_radius, inset_angle: s.inset_angle,
    direction: s.direction as c::PivotDirection::Type,
    compute_inset_angle: s.compute_inset_angle as u8,
});

impl_c_to_proto!(c::HeadingLineCommand => proto::HeadingLineCommand, s, {
    start_x: s.start_x, start_y: s.start_y, dir_x: s.dir_x, dir_y: s.dir_y,
    line_velocity: s.line_velocity, global_theta: s.global_theta,
    max_vel_colinear: s.max_vel_colinear, max_vel_perp: s.max_vel_perp,
    max_vel_angular: s.max_vel_angular,
    max_accel_colinear: s.max_accel_colinear, max_accel_perp: s.max_accel_perp,
    max_accel_angular: s.max_accel_angular, colinear_start_thresh: s.colinear_start_thresh,
});
impl_proto_to_c!(proto::HeadingLineCommand => c::HeadingLineCommand, s, r, {
    start_x: s.start_x, start_y: s.start_y, dir_x: s.dir_x, dir_y: s.dir_y,
    line_velocity: s.line_velocity, global_theta: s.global_theta,
    max_vel_colinear: s.max_vel_colinear, max_vel_perp: s.max_vel_perp,
    max_vel_angular: s.max_vel_angular,
    max_accel_colinear: s.max_accel_colinear, max_accel_perp: s.max_accel_perp,
    max_accel_angular: s.max_accel_angular, colinear_start_thresh: s.colinear_start_thresh,
});

impl_c_to_proto!(c::PointLineCommand => proto::PointLineCommand, s, {
    start_x: s.start_x, start_y: s.start_y, dir_x: s.dir_x, dir_y: s.dir_y,
    line_velocity: s.line_velocity, target_x: s.target_x, target_y: s.target_y,
    max_vel_colinear: s.max_vel_colinear, max_vel_perp: s.max_vel_perp,
    max_vel_angular: s.max_vel_angular,
    max_accel_colinear: s.max_accel_colinear, max_accel_perp: s.max_accel_perp,
    max_accel_angular: s.max_accel_angular, colinear_start_thresh: s.colinear_start_thresh,
});
impl_proto_to_c!(proto::PointLineCommand => c::PointLineCommand, s, r, {
    start_x: s.start_x, start_y: s.start_y, dir_x: s.dir_x, dir_y: s.dir_y,
    line_velocity: s.line_velocity, target_x: s.target_x, target_y: s.target_y,
    max_vel_colinear: s.max_vel_colinear, max_vel_perp: s.max_vel_perp,
    max_vel_angular: s.max_vel_angular,
    max_accel_colinear: s.max_accel_colinear, max_accel_perp: s.max_accel_perp,
    max_accel_angular: s.max_accel_angular, colinear_start_thresh: s.colinear_start_thresh,
});

// ---------------------------------------------------------------------------
// Motor types
// ---------------------------------------------------------------------------

impl_c_to_proto!(c::CcmVelocityTelemetry => proto::CcmVelocityTelemetry, s, {
    vel_setpoint_rads: s.vel_setpoint_rads,
    wheel_vel_rads:    s.wheel_vel_rads,
});
impl_proto_to_c!(proto::CcmVelocityTelemetry => c::CcmVelocityTelemetry, s, r, {
    vel_setpoint_rads: s.vel_setpoint_rads,
    wheel_vel_rads:    s.wheel_vel_rads,
});

// CcmCurrentTelemetry: uint16_t[20] array ↔ heapless::Vec<u32,20>.
// Too complex for the macro — written manually.
impl From<&c::CcmCurrentTelemetry> for proto::CcmCurrentTelemetry {
    fn from(src: &c::CcmCurrentTelemetry) -> Self {
        let mut samples = heapless::Vec::<u32, 20>::new();
        for &s in src.current_samples_ma.iter() {
            let _ = samples.push(s as u32);
        }
        Self {
            bus_voltage_mv:       src.bus_voltage_mv as u32,
            motor_voltage_cmd_mv: src.motor_voltage_cmd_mv as u32,
            current_setpoint_ma:  src.current_setpoint_ma as i32,
            hall_vel_est_drads:   src.hall_vel_est_drads as i32,
            current_samples_ma:   samples,
        }
    }
}

impl From<&proto::CcmCurrentTelemetry> for c::CcmCurrentTelemetry {
    fn from(src: &proto::CcmCurrentTelemetry) -> Self {
        let mut arr = [0u16; 20];
        for (i, &s) in src.current_samples_ma.iter().enumerate().take(20) {
            arr[i] = s as u16;
        }
        Self {
            bus_voltage_mv:       src.bus_voltage_mv as u16,
            motor_voltage_cmd_mv: src.motor_voltage_cmd_mv as u16,
            current_setpoint_ma:  src.current_setpoint_ma as i16,
            hall_vel_est_drads:   src.hall_vel_est_drads as i16,
            current_samples_ma:   arr,
        }
    }
}

// CcmTelemetry: 16-bit bitfield (_bitfield_1) + submessage fields (require setters).
// Bulk-reads/writes all 16 error bits in a single get(0,16)/set(0,16) call.
impl From<&c::CcmTelemetry> for proto::CcmTelemetry {
    fn from(src: &c::CcmTelemetry) -> Self {
        let mut out = proto::CcmTelemetry::default();
        out.error_flags         = src._bitfield_1.get(0, 16) as u32;
        out.motion_control_type = proto::CcmMotionControlType(src.motion_control_type as i32);
        out.gain_stage_index    = src.gain_stage_index() as u32;
        out.set_current_telem((&src.current_telemetry).into());
        out.set_velocity_telem((&src.velocity_telemetry).into());
        out
    }
}

impl From<&proto::CcmTelemetry> for c::CcmTelemetry {
    fn from(src: &proto::CcmTelemetry) -> Self {
        let mut out = c::CcmTelemetry::default();
        out._bitfield_1.set(0, 16, src.error_flags as u64);
        out.motion_control_type = src.motion_control_type.0 as c::CcmMotionControlType::Type;
        out.set_gain_stage_index(src.gain_stage_index as u8);
        out.current_telemetry   = (&src.current_telem).into();
        out.velocity_telemetry  = (&src.velocity_telem).into();
        out
    }
}

// ---------------------------------------------------------------------------
// Power types
// ---------------------------------------------------------------------------

// BatteryInfo: _bitfield_1 holds 7 status flags; all other fields are direct scalars.
impl From<&c::BatteryInfo> for proto::BatteryInfo {
    fn from(src: &c::BatteryInfo) -> Self {
        Self {
            status_flags:  src._bitfield_1.get(0, 7) as u32,
            battery_mv:    src.battery_mv as u32,
            cell1_mv:      src.cell1_mv as u32,
            cell2_mv:      src.cell2_mv as u32,
            cell3_mv:      src.cell3_mv as u32,
            cell4_mv:      src.cell4_mv as u32,
            cell5_mv:      src.cell5_mv as u32,
            cell6_mv:      src.cell6_mv as u32,
            battery_pct:   src.battery_pct as u32,
            cell1_pct:     src.cell1_pct as u32,
            cell2_pct:     src.cell2_pct as u32,
            cell3_pct:     src.cell3_pct as u32,
            cell4_pct:     src.cell4_pct as u32,
            cell5_pct:     src.cell5_pct as u32,
            cell6_pct:     src.cell6_pct as u32,
        }
    }
}

impl From<&proto::BatteryInfo> for c::BatteryInfo {
    fn from(src: &proto::BatteryInfo) -> Self {
        let mut out = c::BatteryInfo::default();
        out._bitfield_1.set(0, 7, src.status_flags as u64);
        out.battery_mv    = src.battery_mv as u16;
        out.cell1_mv      = src.cell1_mv as u16;
        out.cell2_mv      = src.cell2_mv as u16;
        out.cell3_mv      = src.cell3_mv as u16;
        out.cell4_mv      = src.cell4_mv as u16;
        out.cell5_mv      = src.cell5_mv as u16;
        out.cell6_mv      = src.cell6_mv as u16;
        out.battery_pct   = src.battery_pct as u8;
        out.cell1_pct     = src.cell1_pct as u8;
        out.cell2_pct     = src.cell2_pct as u8;
        out.cell3_pct     = src.cell3_pct as u8;
        out.cell4_pct     = src.cell4_pct as u8;
        out.cell5_pct     = src.cell5_pct as u8;
        out.cell6_pct     = src.cell6_pct as u8;
        out
    }
}

// PowerTelemetry: _bitfield_1 holds 6 status flags + submessage battery_info.
impl From<&c::PowerTelemetry> for proto::PowerTelemetry {
    fn from(src: &c::PowerTelemetry) -> Self {
        let mut out = proto::PowerTelemetry::default();
        out.status_flags = src._bitfield_1.get(0, 6) as u32;
        out.set_battery_info((&src.battery_info).into());
        out
    }
}

impl From<&proto::PowerTelemetry> for c::PowerTelemetry {
    fn from(src: &proto::PowerTelemetry) -> Self {
        let mut out = c::PowerTelemetry::default();
        out._bitfield_1.set(0, 6, src.status_flags as u64);
        out.battery_info = (&src.battery_info).into();
        out
    }
}

// ---------------------------------------------------------------------------
// Kicker types
// ---------------------------------------------------------------------------

// KickerTelemetry: _bitfield_1 holds 7 status flags; kicker_image_hash is [u8; 4]
// in C but fixed32 (u32, LE) in proto; dribbler_motor requires setter.
impl From<&c::KickerTelemetry> for proto::KickerTelemetry {
    fn from(src: &c::KickerTelemetry) -> Self {
        let mut out = proto::KickerTelemetry::default();
        out.status_flags       = src._bitfield_1.get(0, 7) as u32;
        out.charge_pct         = src.charge_pct as u32;
        out.rail_voltage       = src.rail_voltage;
        out.battery_voltage    = src.battery_voltage;
        out.kicker_image_hash  = u32::from_le_bytes(src.kicker_image_hash);
        out.set_dribbler_motor((&src.dribbler_motor).into());
        out
    }
}

impl From<&proto::KickerTelemetry> for c::KickerTelemetry {
    fn from(src: &proto::KickerTelemetry) -> Self {
        let mut out = c::KickerTelemetry::default();
        out._bitfield_1.set(0, 7, src.status_flags as u64);
        out.charge_pct         = src.charge_pct as u16;
        out.rail_voltage       = src.rail_voltage;
        out.battery_voltage    = src.battery_voltage;
        out.kicker_image_hash  = src.kicker_image_hash.to_le_bytes();
        out.dribbler_motor     = (&src.dribbler_motor).into();
        out
    }
}

// ---------------------------------------------------------------------------
// Body control extended telemetry
// ---------------------------------------------------------------------------

// BodyControlExtendedTelemetry: body_control_mode drives unsafe union access for
// maneuver_echo; vision_update is in _bitfield_1 bit 0; all float arrays → flat fields.
impl From<&c::BodyControlExtendedTelemetry> for proto::BodyControlExtendedTelemetry {
    fn from(src: &c::BodyControlExtendedTelemetry) -> Self {
        use c::BodyControlMode as BCM;
        // SAFETY: body_control_mode is the discriminant written alongside the union;
        // we only access the maneuver arm that matches the discriminant value.
        let maneuver_echo = unsafe {
            match src.body_control_mode {
                BCM::BCM_GLOBAL_POSITION =>
                    Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::GlobalPos(
                        (&src.maneuver.global_pos.cmd_echo).into()
                    )),
                BCM::BCM_GLOBAL_VELOCITY =>
                    Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::GlobalVel(
                        (&src.maneuver.global_vel.cmd_echo).into()
                    )),
                BCM::BCM_LOCAL_VELOCITY =>
                    Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::LocalVel(
                        (&src.maneuver.local_vel.cmd_echo).into()
                    )),
                BCM::BCM_GLOBAL_ACCEL =>
                    Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::GlobalAcc(
                        (&src.maneuver.global_acc.cmd_echo).into()
                    )),
                BCM::BCM_LOCAL_ACCEL =>
                    Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::LocalAcc(
                        (&src.maneuver.local_acc.cmd_echo).into()
                    )),
                BCM::BCM_HEADING_PIVOT =>
                    Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::HeadingPivot(
                        (&src.maneuver.heading_pivot.cmd_echo).into()
                    )),
                BCM::BCM_POINT_PIVOT =>
                    Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::PointPivot(
                        (&src.maneuver.point_pivot.cmd_echo).into()
                    )),
                BCM::BCM_HEADING_LINE =>
                    Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::HeadingLine(
                        (&src.maneuver.heading_line.cmd_echo).into()
                    )),
                BCM::BCM_POINT_LINE =>
                    Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::PointLine(
                        (&src.maneuver.point_line.cmd_echo).into()
                    )),
                BCM::BCM_OFF | BCM::BCM_ESTOP_BRAKE => None,
                _ => {
                    debug_assert!(false, "unknown body_control_mode: {}", src.body_control_mode);
                    None
                }
            }
        };
        Self {
            body_control_mode:        proto::BodyControlMode(src.body_control_mode as i32),
            vision_update:            src.vision_update() != 0,
            maneuver_echo,
            imu_gyro_x:               src.imu_gyro[0],
            imu_gyro_y:               src.imu_gyro[1],
            imu_gyro_z:               src.imu_gyro[2],
            imu_accel_x:              src.imu_accel[0],
            imu_accel_y:              src.imu_accel[1],
            imu_accel_z:              src.imu_accel[2],
            vision_pose_x:            src.vision_pose[0],
            vision_pose_y:            src.vision_pose[1],
            vision_pose_theta:        src.vision_pose[2],
            traj_pos_x:               src.body_traj_pos[0],
            traj_pos_y:               src.body_traj_pos[1],
            traj_pos_theta:           src.body_traj_pos[2],
            traj_vel_x:               src.body_traj_vel[0],
            traj_vel_y:               src.body_traj_vel[1],
            traj_vel_omega:           src.body_traj_vel[2],
            kf_pred_pos_x:            src.kf_body_pos_prediction[0],
            kf_pred_pos_y:            src.kf_body_pos_prediction[1],
            kf_pred_pos_theta:        src.kf_body_pos_prediction[2],
            kf_pred_vel_x:            src.kf_body_vel_prediction[0],
            kf_pred_vel_y:            src.kf_body_vel_prediction[1],
            kf_pred_vel_omega:        src.kf_body_vel_prediction[2],
            kf_est_pos_x:             src.kf_body_pos_estimate[0],
            kf_est_pos_y:             src.kf_body_pos_estimate[1],
            kf_est_pos_theta:         src.kf_body_pos_estimate[2],
            kf_est_vel_x:             src.kf_body_vel_estimate[0],
            kf_est_vel_y:             src.kf_body_vel_estimate[1],
            kf_est_vel_omega:         src.kf_body_vel_estimate[2],
            body_vel_u_x:             src.body_vel_u[0],
            body_vel_u_y:             src.body_vel_u[1],
            body_vel_u_omega:         src.body_vel_u[2],
            body_accel_u_x:           src.body_accel_u[0],
            body_accel_u_y:           src.body_accel_u[1],
            body_accel_u_omega:       src.body_accel_u[2],
            body_accel_u_fric_x:      src.body_accel_u_fric_comp[0],
            body_accel_u_fric_y:      src.body_accel_u_fric_comp[1],
            body_accel_u_fric_omega:  src.body_accel_u_fric_comp[2],
        }
    }
}

impl From<&proto::BodyControlExtendedTelemetry> for c::BodyControlExtendedTelemetry {
    fn from(src: &proto::BodyControlExtendedTelemetry) -> Self {
        use c::BodyControlMode as BCM;
        let mut out = c::BodyControlExtendedTelemetry::default();
        out.body_control_mode = src.body_control_mode.0 as c::BodyControlMode::Type;
        out.set_vision_update(src.vision_update as u8);
        match &src.maneuver_echo {
            Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::GlobalPos(cmd)) => {
                out.maneuver.global_pos.cmd_echo = cmd.into();
                out.body_control_mode = BCM::BCM_GLOBAL_POSITION;
            }
            Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::GlobalVel(cmd)) => {
                out.maneuver.global_vel.cmd_echo = cmd.into();
                out.body_control_mode = BCM::BCM_GLOBAL_VELOCITY;
            }
            Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::LocalVel(cmd)) => {
                out.maneuver.local_vel.cmd_echo = cmd.into();
                out.body_control_mode = BCM::BCM_LOCAL_VELOCITY;
            }
            Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::GlobalAcc(cmd)) => {
                out.maneuver.global_acc.cmd_echo = cmd.into();
                out.body_control_mode = BCM::BCM_GLOBAL_ACCEL;
            }
            Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::LocalAcc(cmd)) => {
                out.maneuver.local_acc.cmd_echo = cmd.into();
                out.body_control_mode = BCM::BCM_LOCAL_ACCEL;
            }
            Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::HeadingPivot(cmd)) => {
                out.maneuver.heading_pivot.cmd_echo = cmd.into();
                out.body_control_mode = BCM::BCM_HEADING_PIVOT;
            }
            Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::PointPivot(cmd)) => {
                out.maneuver.point_pivot.cmd_echo = cmd.into();
                out.body_control_mode = BCM::BCM_POINT_PIVOT;
            }
            Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::HeadingLine(cmd)) => {
                out.maneuver.heading_line.cmd_echo = cmd.into();
                out.body_control_mode = BCM::BCM_HEADING_LINE;
            }
            Some(proto::BodyControlExtendedTelemetry_::ManeuverEcho::PointLine(cmd)) => {
                out.maneuver.point_line.cmd_echo = cmd.into();
                out.body_control_mode = BCM::BCM_POINT_LINE;
            }
            None => {}
        }
        out.imu_gyro[0]                 = src.imu_gyro_x;
        out.imu_gyro[1]                 = src.imu_gyro_y;
        out.imu_gyro[2]                 = src.imu_gyro_z;
        out.imu_accel[0]                = src.imu_accel_x;
        out.imu_accel[1]                = src.imu_accel_y;
        out.imu_accel[2]                = src.imu_accel_z;
        out.vision_pose[0]              = src.vision_pose_x;
        out.vision_pose[1]              = src.vision_pose_y;
        out.vision_pose[2]              = src.vision_pose_theta;
        out.body_traj_pos[0]            = src.traj_pos_x;
        out.body_traj_pos[1]            = src.traj_pos_y;
        out.body_traj_pos[2]            = src.traj_pos_theta;
        out.body_traj_vel[0]            = src.traj_vel_x;
        out.body_traj_vel[1]            = src.traj_vel_y;
        out.body_traj_vel[2]            = src.traj_vel_omega;
        out.kf_body_pos_prediction[0]   = src.kf_pred_pos_x;
        out.kf_body_pos_prediction[1]   = src.kf_pred_pos_y;
        out.kf_body_pos_prediction[2]   = src.kf_pred_pos_theta;
        out.kf_body_vel_prediction[0]   = src.kf_pred_vel_x;
        out.kf_body_vel_prediction[1]   = src.kf_pred_vel_y;
        out.kf_body_vel_prediction[2]   = src.kf_pred_vel_omega;
        out.kf_body_pos_estimate[0]     = src.kf_est_pos_x;
        out.kf_body_pos_estimate[1]     = src.kf_est_pos_y;
        out.kf_body_pos_estimate[2]     = src.kf_est_pos_theta;
        out.kf_body_vel_estimate[0]     = src.kf_est_vel_x;
        out.kf_body_vel_estimate[1]     = src.kf_est_vel_y;
        out.kf_body_vel_estimate[2]     = src.kf_est_vel_omega;
        out.body_vel_u[0]               = src.body_vel_u_x;
        out.body_vel_u[1]               = src.body_vel_u_y;
        out.body_vel_u[2]               = src.body_vel_u_omega;
        out.body_accel_u[0]             = src.body_accel_u_x;
        out.body_accel_u[1]             = src.body_accel_u_y;
        out.body_accel_u[2]             = src.body_accel_u_omega;
        out.body_accel_u_fric_comp[0]   = src.body_accel_u_fric_x;
        out.body_accel_u_fric_comp[1]   = src.body_accel_u_fric_y;
        out.body_accel_u_fric_comp[2]   = src.body_accel_u_fric_omega;
        out
    }
}

// ---------------------------------------------------------------------------
// Top-level packets
// ---------------------------------------------------------------------------

// ExtendedTelemetry: all submessage fields; C uses body_control_telemetry, proto uses
// body_control_telem. The `_has` bits are set via set_X() setters.
impl From<&c::ExtendedTelemetry> for proto::ExtendedTelemetry {
    fn from(src: &c::ExtendedTelemetry) -> Self {
        let mut out = proto::ExtendedTelemetry::default();
        out.timestamp_us_lo = src.timestamp_us_lo;
        out.timestamp_us_hi = src.timestamp_us_hi;
        out.set_power_status((&src.power_status).into());
        out.set_front_left_motor((&src.front_left_motor).into());
        out.set_back_left_motor((&src.back_left_motor).into());
        out.set_back_right_motor((&src.back_right_motor).into());
        out.set_front_right_motor((&src.front_right_motor).into());
        out.set_body_control_telem((&src.body_control_telemetry).into());
        out.set_kicker_status((&src.kicker_status).into());
        out
    }
}

impl From<&proto::ExtendedTelemetry> for c::ExtendedTelemetry {
    fn from(src: &proto::ExtendedTelemetry) -> Self {
        c::ExtendedTelemetry {
            timestamp_us_lo:         src.timestamp_us_lo,
            timestamp_us_hi:         src.timestamp_us_hi,
            power_status:            (&src.power_status).into(),
            front_left_motor:        (&src.front_left_motor).into(),
            back_left_motor:         (&src.back_left_motor).into(),
            back_right_motor:        (&src.back_right_motor).into(),
            front_right_motor:       (&src.front_right_motor).into(),
            body_control_telemetry:  (&src.body_control_telem).into(),
            kicker_status:           (&src.kicker_status).into(),
        }
    }
}

// BasicControl: bitfield holds 9 control flags; vision_position_update[3] → vision_x/y/theta;
// body_control_mode + cmd union with unsafe; BCM_ESTOP_BRAKE ↔ estop_brake bool.
impl From<&c::BasicControl> for proto::BasicControl {
    fn from(src: &c::BasicControl) -> Self {
        use c::BodyControlMode as BCM;
        let mut out = proto::BasicControl::default();
        out.request_shutdown          = src.request_shutdown() != 0;
        out.reboot_robot              = src.reboot_robot() != 0;
        out.game_state_in_stop        = src.game_state_in_stop() != 0;
        out.game_state_in_halt        = src.game_state_in_halt() != 0;
        out.emergency_stop            = src.emergency_stop() != 0;
        out.wheel_vel_control_enabled = src.wheel_vel_control_enabled() != 0;
        out.wheel_torque_control_enabled = src.wheel_torque_control_enabled() != 0;
        out.vision_update_valid       = src.vision_update() != 0;
        out.reset_controller          = src.reset_controller() != 0;
        if src.vision_update() != 0 {
            out.vision_x     = src.vision_position_update[0];
            out.vision_y     = src.vision_position_update[1];
            out.vision_theta = src.vision_position_update[2];
        }
        out.kick_request    = proto::KickRequest(src.kick_request as i32);
        out.kick_vel        = src.kick_vel;
        out.dribbler_mode   = proto::DribblerMode(src.dribbler_mode as i32);
        out.dribbler_setpoint = src.dribbler_setpoint;
        out.play_song       = src.play_song as u32;
        // SAFETY: body_control_mode is the discriminant written alongside the union;
        // we only access the cmd arm that matches the discriminant value.
        out.cmd = unsafe {
            match src.body_control_mode {
                BCM::BCM_ESTOP_BRAKE => { out.estop_brake = true; None }
                BCM::BCM_GLOBAL_POSITION => Some(proto::BasicControl_::Cmd::GlobalPos((&src.cmd.global_pos).into())),
                BCM::BCM_GLOBAL_VELOCITY => Some(proto::BasicControl_::Cmd::GlobalVel((&src.cmd.global_vel).into())),
                BCM::BCM_LOCAL_VELOCITY  => Some(proto::BasicControl_::Cmd::LocalVel((&src.cmd.local_vel).into())),
                BCM::BCM_GLOBAL_ACCEL    => Some(proto::BasicControl_::Cmd::GlobalAcc((&src.cmd.global_acc).into())),
                BCM::BCM_LOCAL_ACCEL     => Some(proto::BasicControl_::Cmd::LocalAcc((&src.cmd.local_acc).into())),
                BCM::BCM_HEADING_PIVOT   => Some(proto::BasicControl_::Cmd::HeadingPivot((&src.cmd.heading_pivot).into())),
                BCM::BCM_POINT_PIVOT     => Some(proto::BasicControl_::Cmd::PointPivot((&src.cmd.point_pivot).into())),
                BCM::BCM_HEADING_LINE    => Some(proto::BasicControl_::Cmd::HeadingLine((&src.cmd.heading_line).into())),
                BCM::BCM_POINT_LINE      => Some(proto::BasicControl_::Cmd::PointLine((&src.cmd.point_line).into())),
                BCM::BCM_OFF => None,
                _ => {
                    debug_assert!(false, "unknown body_control_mode: {}", src.body_control_mode);
                    None
                }
            }
        };
        out
    }
}

impl From<&proto::BasicControl> for c::BasicControl {
    fn from(src: &proto::BasicControl) -> Self {
        use c::BodyControlMode as BCM;
        let mut out = c::BasicControl::default();
        out.set_request_shutdown(src.request_shutdown as u32);
        out.set_reboot_robot(src.reboot_robot as u32);
        out.set_game_state_in_stop(src.game_state_in_stop as u32);
        out.set_game_state_in_halt(src.game_state_in_halt as u32);
        out.set_emergency_stop(src.emergency_stop as u32);
        out.set_wheel_vel_control_enabled(src.wheel_vel_control_enabled as u32);
        out.set_wheel_torque_control_enabled(src.wheel_torque_control_enabled as u32);
        out.set_vision_update(src.vision_update_valid as u32);
        out.set_reset_controller(src.reset_controller as u32);
        if src.vision_update_valid {
            out.vision_position_update[0] = src.vision_x;
            out.vision_position_update[1] = src.vision_y;
            out.vision_position_update[2] = src.vision_theta;
        }
        out.kick_request      = src.kick_request.0 as c::KickRequest::Type;
        out.kick_vel          = src.kick_vel;
        out.dribbler_mode     = src.dribbler_mode.0 as c::DribblerCommand::Type;
        out.dribbler_setpoint = src.dribbler_setpoint;
        out.play_song         = src.play_song as u8;
        if src.estop_brake {
            out.body_control_mode = BCM::BCM_ESTOP_BRAKE;
        } else {
            match &src.cmd {
                None => out.body_control_mode = BCM::BCM_OFF,
                Some(proto::BasicControl_::Cmd::GlobalPos(cmd)) => {
                    out.body_control_mode = BCM::BCM_GLOBAL_POSITION;
                    out.cmd.global_pos    = cmd.into();
                }
                Some(proto::BasicControl_::Cmd::GlobalVel(cmd)) => {
                    out.body_control_mode = BCM::BCM_GLOBAL_VELOCITY;
                    out.cmd.global_vel    = cmd.into();
                }
                Some(proto::BasicControl_::Cmd::LocalVel(cmd)) => {
                    out.body_control_mode = BCM::BCM_LOCAL_VELOCITY;
                    out.cmd.local_vel     = cmd.into();
                }
                Some(proto::BasicControl_::Cmd::GlobalAcc(cmd)) => {
                    out.body_control_mode = BCM::BCM_GLOBAL_ACCEL;
                    out.cmd.global_acc    = cmd.into();
                }
                Some(proto::BasicControl_::Cmd::LocalAcc(cmd)) => {
                    out.body_control_mode = BCM::BCM_LOCAL_ACCEL;
                    out.cmd.local_acc     = cmd.into();
                }
                Some(proto::BasicControl_::Cmd::HeadingPivot(cmd)) => {
                    out.body_control_mode = BCM::BCM_HEADING_PIVOT;
                    out.cmd.heading_pivot = cmd.into();
                }
                Some(proto::BasicControl_::Cmd::PointPivot(cmd)) => {
                    out.body_control_mode = BCM::BCM_POINT_PIVOT;
                    out.cmd.point_pivot   = cmd.into();
                }
                Some(proto::BasicControl_::Cmd::HeadingLine(cmd)) => {
                    out.body_control_mode = BCM::BCM_HEADING_LINE;
                    out.cmd.heading_line  = cmd.into();
                }
                Some(proto::BasicControl_::Cmd::PointLine(cmd)) => {
                    out.body_control_mode = BCM::BCM_POINT_LINE;
                    out.cmd.point_line    = cmd.into();
                }

            }
        }
        out
    }
}

// BasicTelemetry: _bitfield_1 is 27-bit status bitfield; kf arrays are i16[3] → sint32 fields;
// C uses byte sequence numbers (u8) while proto uses u32.
// Note: C's control_telem (BodyControlTelemetry) is not in the proto BasicTelemetry.
impl From<&c::BasicTelemetry> for proto::BasicTelemetry {
    fn from(src: &c::BasicTelemetry) -> Self {
        Self {
            tx_seq_num:           src.transmission_sequence_number as u32,
            ctrl_seq_num:         src.control_data_sequence_number as u32,
            body_control_mode:    proto::BodyControlMode(src.body_control_mode as i32),
            status_flags:         src._bitfield_1.get(0, 27) as u32,
            battery_percent:      src.battery_percent as u32,
            kicker_charge_percent: src.kicker_charge_percent as u32,
            kf_pos_x:             src.kf_body_pos_estimate[0] as i32,
            kf_pos_y:             src.kf_body_pos_estimate[1] as i32,
            kf_pos_theta:         src.kf_body_pos_estimate[2] as i32,
            kf_vel_x:             src.kf_body_vel_estimate[0] as i32,
            kf_vel_y:             src.kf_body_vel_estimate[1] as i32,
            kf_vel_omega:         src.kf_body_vel_estimate[2] as i32,
        }
    }
}

impl From<&proto::BasicTelemetry> for c::BasicTelemetry {
    fn from(src: &proto::BasicTelemetry) -> Self {
        let mut out = c::BasicTelemetry::default();
        out.transmission_sequence_number  = src.tx_seq_num as u8;
        out.control_data_sequence_number  = src.ctrl_seq_num as u8;
        out.body_control_mode             = src.body_control_mode.0 as c::BodyControlMode::Type;
        out._bitfield_1.set(0, 27, src.status_flags as u64);
        out.battery_percent               = src.battery_percent as u16;
        out.kicker_charge_percent         = src.kicker_charge_percent as u16;
        out.kf_body_pos_estimate[0]       = src.kf_pos_x as i16;
        out.kf_body_pos_estimate[1]       = src.kf_pos_y as i16;
        out.kf_body_pos_estimate[2]       = src.kf_pos_theta as i16;
        out.kf_body_vel_estimate[0]       = src.kf_vel_x as i16;
        out.kf_body_vel_estimate[1]       = src.kf_vel_y as i16;
        out.kf_body_vel_estimate[2]       = src.kf_vel_omega as i16;
        out
    }
}

// ---------------------------------------------------------------------------
// Bitfield position test — asserts that _bitfield_1.get(0,16) matches the
// individual accessor positions that bindgen generates. Catches layout changes.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::c;

    // Failure modes per invariant:
    //   • Setter renamed/removed  → compile error (method not found)
    //   • New bit added to C header → test failure (OR sum < expected mask)
    //   • $width wrong              → test failure (mask mismatch)
    //   • Bit reordered near top    → last-bit assertion fails
    // Only case NOT caught at compile time: new bit added without updating this list.
    // That produces a test failure, which is the best achievable without proc macros.
    macro_rules! assert_bitfield_exhaustive {
        (
            $T:ty,
            last: $last_setter:ident @ $last_bit:expr,
            width: $width:expr,
            [$($setter:ident),+ $(,)?]
        ) => {{
            let mut v = <$T>::default();
            v.$last_setter(1);
            assert_eq!(
                v._bitfield_1.get(0, $width),
                1u64 << $last_bit,
                concat!(stringify!($last_setter), " must be bit ", stringify!($last_bit)),
            );

            let mut v = <$T>::default();
            $(v.$setter(1);)+
            assert_eq!(
                v._bitfield_1.get(0, $width),
                (1u64 << $width) - 1,
                concat!(stringify!($T), ": setter list doesn't cover all ", stringify!($width), " bits — update list or width"),
            );
        }};
    }

    #[test]
    fn ccm_error_bitfield_positions() {
        assert_bitfield_exhaustive!(
            c::CcmTelemetry,
            last: set_reset_pin @ 15,
            width: 16,
            [
                set_master_error,
                set_hall_power_error,
                set_hall_disconnected_error,
                set_bldc_transition_error,
                set_bldc_commutation_watchdog_error,
                set_enc_disconnected_error,
                set_overcurrent_error,
                set_undervoltage_error,
                set_overvoltage_error,
                set_torque_limited,
                set_control_loop_time_error,
                set_reset_watchdog_independent,
                set_reset_watchdog_window,
                set_reset_low_power,
                set_reset_software,
                set_reset_pin,
            ]
        );
    }

    #[test]
    fn battery_info_bitfield_positions() {
        assert_bitfield_exhaustive!(
            c::BatteryInfo,
            last: set_battery_cell_imbalance_warn @ 6,
            width: 7,
            [
                set_battery_ok,
                set_battery_balance_connected,
                set_battery_low,
                set_battery_critical,
                set_battery_cell_low,
                set_battery_cell_critical,
                set_battery_cell_imbalance_warn,
            ]
        );
    }

    #[test]
    fn power_telemetry_bitfield_positions() {
        assert_bitfield_exhaustive!(
            c::PowerTelemetry,
            last: set_shutdown_requested @ 5,
            width: 6,
            [
                set_power_ok,
                set_power_rail_3v3_ok,
                set_power_rail_5v0_ok,
                set_power_rail_12v0_ok,
                set_high_current_operations_allowed,
                set_shutdown_requested,
            ]
        );
    }

    #[test]
    fn kicker_telemetry_bitfield_positions() {
        assert_bitfield_exhaustive!(
            c::KickerTelemetry,
            last: set_dribbler_fw_loaded @ 6,
            width: 7,
            [
                set_error_detected,
                set_dribbler_error,
                set_power_down_requested,
                set_power_down_complete,
                set_ball_detected,
                set_charge_full,
                set_dribbler_fw_loaded,
            ]
        );
    }

    #[test]
    fn basic_telemetry_bitfield_positions() {
        assert_bitfield_exhaustive!(
            c::BasicTelemetry,
            last: set_controller_reset @ 26,
            width: 27,
            [
                set_power_error,
                set_power_board_error,
                set_battery_error,
                set_battery_low,
                set_battery_crit,
                set_shutdown_pending,
                set_tipped_error,
                set_breakbeam_error,
                set_breakbeam_ball_detected,
                set_accelerometer_0_error,
                set_accelerometer_1_error,
                set_gyroscope_0_error,
                set_gyroscope_1_error,
                set_motor_fl_general_error,
                set_motor_fl_hall_error,
                set_motor_bl_general_error,
                set_motor_bl_hall_error,
                set_motor_br_general_error,
                set_motor_br_hall_error,
                set_motor_fr_general_error,
                set_motor_fr_hall_error,
                set_motor_drib_general_error,
                set_motor_drib_hall_error,
                set_kicker_board_error,
                set_chipper_available,
                set_kicker_available,
                set_controller_reset,
            ]
        );
        // Reserved bits above position 26 must not bleed when all status bits set.
        let mut t = c::BasicTelemetry::default();
        t._bitfield_1.set(0, 27, (1u64 << 27) - 1);
        assert_eq!(t._bitfield_1.get(27, 5), 0, "bits 27-31 must not bleed into reserved region");
    }
}
