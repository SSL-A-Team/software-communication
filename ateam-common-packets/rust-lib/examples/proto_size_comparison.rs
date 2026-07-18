// Wire-size comparison: C packed structs vs micropb-encoded protobuf.
//
// Raw sizes are always fixed (sizeof the C struct).
// Proto sizes are computed via MessageEncode::compute_size(), which gives the
// exact byte count of the encoded message without actually allocating a buffer.

use ateam_common_packets::proto_packets::ateam_::{
    BasicControl, BasicControl_, BasicTelemetry, LocalVelocityCommand, GlobalVelocityCommand,
    GlobalPositionCommand, PointLineCommand, KickRequest, DribblerMode,
    ExtendedTelemetry, PowerTelemetry, BatteryInfo, CcmTelemetry,
    CcmCurrentTelemetry, CcmVelocityTelemetry, BodyControlExtendedTelemetry,
    BodyControlExtendedTelemetry_, KickerTelemetry,
    BodyControlMode, CcmMotionControlType,
};
use micropb::{MessageEncode, PbEncoder};

fn main() {
    // ---- Raw struct sizes (from C headers, same on ARM and x86_64 for these fixed-width types) ----
    let raw_basic_control: usize = core::mem::size_of::<ateam_common_packets::bindings::BasicControl>();
    let raw_basic_telemetry: usize = core::mem::size_of::<ateam_common_packets::bindings::BasicTelemetry>();
    let raw_extended_telemetry: usize = core::mem::size_of::<ateam_common_packets::bindings::ExtendedTelemetry>();

    println!("=== Wire Size Comparison ===");
    println!("Raw C struct sizes (always fixed):");
    println!("  BasicControl:     {} bytes", raw_basic_control);
    println!("  BasicTelemetry:   {} bytes", raw_basic_telemetry);
    println!("  ExtendedTelemetry:{} bytes", raw_extended_telemetry);
    println!();

    // ---- BasicControl scenarios ----
    println!("=== BasicControl (raw: {} bytes) ===", raw_basic_control);

    // Scenario 1: BCM_OFF — no cmd, no flags (robot idling).
    let ctrl_off = BasicControl::default();
    print_control_row("BCM_OFF (empty/idle)", ctrl_off, raw_basic_control);

    // Scenario 2: BCM_ESTOP_BRAKE — just the brake flag.
    let ctrl_estop = BasicControl {
        estop_brake: true,
        ..Default::default()
    };
    print_control_row("BCM_ESTOP_BRAKE", ctrl_estop, raw_basic_control);

    // Scenario 3: BCM_LOCAL_VELOCITY — typical motion command, no kick/dribble.
    let ctrl_local_vel = BasicControl {
        cmd: Some(BasicControl_::Cmd::LocalVel(LocalVelocityCommand {
            local_xd: 0.5,
            local_yd: 0.0,
            local_omega: 0.3,
            max_linear_acc: 3.0,
            max_angular_acc: 5.0,
        })),
        ..Default::default()
    };
    print_control_row("BCM_LOCAL_VELOCITY (no kick/drib)", ctrl_local_vel, raw_basic_control);

    // Scenario 4: BCM_LOCAL_VELOCITY + active kicking + dribbler.
    let ctrl_kick = BasicControl {
        cmd: Some(BasicControl_::Cmd::LocalVel(LocalVelocityCommand {
            local_xd: 0.5,
            local_yd: 0.0,
            local_omega: 0.1,
            max_linear_acc: 3.0,
            max_angular_acc: 5.0,
        })),
        kick_request: KickRequest::KrKickNow as _,
        kick_vel: 4.5,
        dribbler_mode: DribblerMode::DmDribble as _,
        dribbler_setpoint: 100.0,
        ..Default::default()
    };
    print_control_row("BCM_LOCAL_VELOCITY + kick + dribbler", ctrl_kick, raw_basic_control);

    // Scenario 5: BCM_GLOBAL_VELOCITY with vision update.
    let ctrl_vision = BasicControl {
        cmd: Some(BasicControl_::Cmd::GlobalVel(GlobalVelocityCommand {
            global_xd: 1.0,
            global_yd: 0.5,
            global_omega: 0.2,
            max_linear_acc: 2.0,
            max_angular_acc: 4.0,
        })),
        vision_update_valid: true,
        vision_x: 3.14,
        vision_y: 2.71,
        vision_theta: 1.57,
        ..Default::default()
    };
    print_control_row("BCM_GLOBAL_VELOCITY + vision update", ctrl_vision, raw_basic_control);

    // Scenario 6: BCM_GLOBAL_POSITION (7-float maneuver, common in AI software).
    let ctrl_pos = BasicControl {
        cmd: Some(BasicControl_::Cmd::GlobalPos(GlobalPositionCommand {
            global_x: 1.0,
            global_y: 2.0,
            global_theta: 0.78,
            max_linear_vel: 2.0,
            max_angular_vel: 6.0,
            max_linear_acc: 3.0,
            max_angular_acc: 8.0,
        })),
        ..Default::default()
    };
    print_control_row("BCM_GLOBAL_POSITION", ctrl_pos, raw_basic_control);

    // Scenario 7: BCM_POINT_LINE — largest maneuver (14 floats), all fields set (worst case).
    let ctrl_pointline_full = BasicControl {
        cmd: Some(BasicControl_::Cmd::PointLine(PointLineCommand {
            start_x: 0.1,
            start_y: 0.2,
            dir_x: 1.0,
            dir_y: 0.0,
            line_velocity: 1.5,
            target_x: 3.0,
            target_y: 2.0,
            max_vel_colinear: 2.0,
            max_vel_perp: 1.0,
            max_vel_angular: 6.0,
            max_accel_colinear: 3.0,
            max_accel_perp: 2.0,
            max_accel_angular: 8.0,
            colinear_start_thresh: 0.05,
        })),
        kick_request: KickRequest::KrKickNow as _,
        kick_vel: 5.0,
        dribbler_mode: DribblerMode::DmDribble as _,
        dribbler_setpoint: 100.0,
        request_shutdown: false,
        game_state_in_stop: true,
        emergency_stop: false,
        wheel_vel_control_enabled: true,
        ..Default::default()
    };
    print_control_row("BCM_POINT_LINE (14-float cmd, active kick, 2 flags)", ctrl_pointline_full, raw_basic_control);

    // Scenario 8: All flags set + BCM_POINT_LINE + vision = absolute worst case.
    let ctrl_worst = BasicControl {
        cmd: Some(BasicControl_::Cmd::PointLine(PointLineCommand {
            start_x: 0.1, start_y: 0.2, dir_x: 1.0, dir_y: 0.0,
            line_velocity: 1.5, target_x: 3.0, target_y: 2.0,
            max_vel_colinear: 2.0, max_vel_perp: 1.0, max_vel_angular: 6.0,
            max_accel_colinear: 3.0, max_accel_perp: 2.0, max_accel_angular: 8.0,
            colinear_start_thresh: 0.05,
        })),
        estop_brake: true,
        kick_request: KickRequest::KrKickNow as _,
        kick_vel: 5.0,
        dribbler_mode: DribblerMode::DmDribble as _,
        dribbler_setpoint: 100.0,
        request_shutdown: true,
        reboot_robot: true,
        game_state_in_stop: true,
        game_state_in_halt: true,
        emergency_stop: true,
        wheel_vel_control_enabled: true,
        wheel_torque_control_enabled: true,
        reset_controller: true,
        vision_update_valid: true,
        vision_x: 1.0, vision_y: 2.0, vision_theta: 0.5,
        play_song: 1,
    };
    print_control_row("Worst case (all flags, point_line, vision)", ctrl_worst, raw_basic_control);

    println!();

    // ---- BasicTelemetry scenarios ----
    // status_flags is a bitmask — see BasicTelemetryFlag in telemetry.proto for bit definitions.
    // bits 0-22: error/status bits; bit 23: kicker_board_error; bits 24-25: availability flags;
    // bit 26: controller_reset.
    println!("=== BasicTelemetry (raw: {} bytes) ===", raw_basic_telemetry);

    // Scenario 1: Minimal — BCM_OFF, no state (e.g., robot just powered on).
    let telem_minimal = BasicTelemetry {
        tx_seq_num: 1,
        ..Default::default()
    };
    print_telemetry_row("Minimal (BCM_OFF, no state)", telem_minimal, raw_basic_telemetry);

    // Scenario 2: Nominal — active robot, no errors, KF state populated.
    // bits 24+25 set = CHIPPER_AVAILABLE | KICKER_AVAILABLE = 0x03000000
    let telem_nominal = BasicTelemetry {
        tx_seq_num: 42,
        ctrl_seq_num: 41,
        body_control_mode: BodyControlMode(12), // BCM_LOCAL_VELOCITY
        status_flags: 0x03000000,               // chipper_available | kicker_available
        battery_percent: 7500,
        kicker_charge_percent: 9000,
        kf_pos_x: 1500,     // 1.5 m
        kf_pos_y: -500,     // -0.5 m
        kf_pos_theta: 1000, // ~1 rad
        kf_vel_x: 500,      // 0.5 m/s
        kf_vel_y: -200,     // -0.2 m/s
        kf_vel_omega: 100,  // 0.1 rad/s
    };
    print_telemetry_row("Nominal (BCM_LOCAL_VEL, no errors, KF populated)", telem_nominal, raw_basic_telemetry);

    // Scenario 3: Error state — multiple errors set, kicker unavailable.
    // POWER_ERROR(0) | BATTERY_ERROR(2) | BATTERY_LOW(3) | MOTOR_FL_GENERAL(13) | MOTOR_BL_GENERAL(15)
    // = 1 | 4 | 8 | 8192 | 32768 = 40973
    let telem_errors = BasicTelemetry {
        tx_seq_num: 100,
        ctrl_seq_num: 98,
        status_flags: 40973,
        battery_percent: 1200,
        ..Default::default()
    };
    print_telemetry_row("Error state (5 errors, low battery, kf=0)", telem_errors, raw_basic_telemetry);

    // Scenario 4: All possible fields non-zero (worst case).
    // 0x07FF_FFFF = all 27 bits set (errors 0-22 + kicker_board_error + chipper + kicker + ctrl_reset)
    let telem_worst = BasicTelemetry {
        tx_seq_num: 255,
        ctrl_seq_num: 255,
        body_control_mode: BodyControlMode(31), // BCM_POINT_LINE
        status_flags: 0x07FF_FFFF,
        battery_percent: 10000,
        kicker_charge_percent: 10000,
        kf_pos_x: 9000,
        kf_pos_y: -9000,
        kf_pos_theta: 3142,
        kf_vel_x: 3000,
        kf_vel_y: -3000,
        kf_vel_omega: 6000,
    };
    print_telemetry_row("Worst case (all errors, all fields set)", telem_worst, raw_basic_telemetry);

    println!();

    // ---- ExtendedTelemetry scenarios ----
    println!("=== ExtendedTelemetry (raw: {} bytes) ===", raw_extended_telemetry);

    // Submessage fields require setter methods (not struct literals) so micropb
    // sets the presence bit in _has correctly for encoding.

    // Helper: active motor (no errors, velocity control, 150mA per sample).
    let nominal_motor = || {
        let mut samples = heapless::Vec::<u32, 20>::new();
        for _ in 0..20 { let _ = samples.push(150); }
        let current_telem = CcmCurrentTelemetry {
            bus_voltage_mv: 24000, motor_voltage_cmd_mv: 2400,
            current_setpoint_ma: 500, hall_vel_est_drads: 80,
            current_samples_ma: samples,
        };
        let velocity_telem = CcmVelocityTelemetry { vel_setpoint_rads: 2.5, wheel_vel_rads: 2.45 };
        let mut m = CcmTelemetry::default();
        m.error_flags = 0;
        m.motion_control_type = CcmMotionControlType(4);
        m.gain_stage_index = 2;
        m.set_current_telem(current_telem);
        m.set_velocity_telem(velocity_telem);
        m
    };

    // Helper: failed motor (error flag only, no current/velocity data).
    let failed_motor = || {
        let mut m = CcmTelemetry::default();
        m.error_flags = 0x0001; // master_error
        m
    };

    // Helper: power board nominal (all rails OK, 6S battery at 80%).
    let nominal_power = || {
        let battery = BatteryInfo {
            status_flags: 0x01, battery_mv: 22800,
            cell1_mv: 3800, cell2_mv: 3800, cell3_mv: 3800,
            cell4_mv: 3800, cell5_mv: 3800, cell6_mv: 3800,
            battery_pct: 80,
            cell1_pct: 80, cell2_pct: 80, cell3_pct: 80,
            cell4_pct: 80, cell5_pct: 80, cell6_pct: 80,
        };
        let mut p = PowerTelemetry::default();
        p.status_flags = 0x1F;
        p.set_battery_info(battery);
        p
    };

    // Helper: dribbler sub-motor (velocity/current mode).
    let dribbler_motor = || {
        let current_telem = CcmCurrentTelemetry {
            bus_voltage_mv: 22800, current_setpoint_ma: 2000,
            ..Default::default()
        };
        let velocity_telem = CcmVelocityTelemetry { vel_setpoint_rads: 100.0, wheel_vel_rads: 99.5 };
        let mut m = CcmTelemetry::default();
        m.motion_control_type = CcmMotionControlType(5);
        m.gain_stage_index = 1;
        m.set_current_telem(current_telem);
        m.set_velocity_telem(velocity_telem);
        m
    };

    // Helper: kicker nominal (charged, dribbler active).
    let nominal_kicker = || {
        let mut k = KickerTelemetry::default();
        k.status_flags = 0x20; k.charge_pct = 9000;
        k.rail_voltage = 180.0; k.battery_voltage = 22.8;
        k.kicker_image_hash = 0xDEADBEEF;
        k.set_dribbler_motor(dribbler_motor());
        k
    };

    // Helper: body controller nominal (BCM_LOCAL_VELOCITY, all sensor arrays populated).
    let nominal_body_control = || BodyControlExtendedTelemetry {
        body_control_mode: BodyControlMode(12), // BCM_LOCAL_VELOCITY
        vision_update: true,
        maneuver_echo: Some(BodyControlExtendedTelemetry_::ManeuverEcho::LocalVel(LocalVelocityCommand {
            local_xd: 0.5, local_yd: 0.0, local_omega: 0.3,
            max_linear_acc: 3.0, max_angular_acc: 5.0,
        })),
        imu_gyro_x: 0.01, imu_gyro_y: 0.02, imu_gyro_z: 0.1,
        imu_accel_x: 0.1,  imu_accel_y: 0.05, imu_accel_z: 9.81,
        vision_pose_x: 1.5, vision_pose_y: -0.5, vision_pose_theta: 1.0,
        traj_pos_x: 1.6,   traj_pos_y: -0.4,  traj_pos_theta: 1.1,
        traj_vel_x: 0.5,   traj_vel_y: 0.0,   traj_vel_omega: 0.3,
        kf_pred_pos_x: 1.5, kf_pred_pos_y: -0.5, kf_pred_pos_theta: 1.0,
        kf_pred_vel_x: 0.5, kf_pred_vel_y: 0.0,  kf_pred_vel_omega: 0.3,
        kf_est_pos_x: 1.5,  kf_est_pos_y: -0.5,  kf_est_pos_theta: 1.0,
        kf_est_vel_x: 0.5,  kf_est_vel_y: 0.0,   kf_est_vel_omega: 0.3,
        body_vel_u_x: 0.5,  body_vel_u_y: 0.0,   body_vel_u_omega: 0.3,
        body_accel_u_x: 1.0, body_accel_u_y: 0.0, body_accel_u_omega: 0.5,
        body_accel_u_fric_x: 1.2, body_accel_u_fric_y: 0.1, body_accel_u_fric_omega: 0.5,
    };

    // Scenario 1: All motors failed, robot at rest (very sparse).
    let ext_all_failed = {
        let mut p = PowerTelemetry::default();
        p.status_flags = 0x04; // power_board_error
        let mut e = ExtendedTelemetry::default();
        e.timestamp_us_lo = 1000000;
        e.set_power_status(p);
        e.set_front_left_motor(failed_motor());
        e.set_back_left_motor(failed_motor());
        e.set_back_right_motor(failed_motor());
        e.set_front_right_motor(failed_motor());
        e
    };
    print_ext_row("All motors failed, robot at rest", ext_all_failed, raw_extended_telemetry);

    // Scenario 2: Nominal active robot (all subsystems working, 150mA motor current).
    let ext_nominal = {
        let mut e = ExtendedTelemetry::default();
        e.timestamp_us_lo = 1234567890;
        e.set_power_status(nominal_power());
        e.set_front_left_motor(nominal_motor());
        e.set_back_left_motor(nominal_motor());
        e.set_back_right_motor(nominal_motor());
        e.set_front_right_motor(nominal_motor());
        e.set_body_control_telem(nominal_body_control());
        e.set_kicker_status(nominal_kicker());
        e
    };
    print_ext_row("Nominal (active robot, all sensors, kicker charged)", ext_nominal, raw_extended_telemetry);

    // Scenario 3: Nominal but motors at rest (current samples = 0 → not encoded).
    let motor_zero_current = || {
        let current_telem = CcmCurrentTelemetry {
            bus_voltage_mv: 24000, ..Default::default()
        };
        let velocity_telem = CcmVelocityTelemetry::default();
        let mut m = CcmTelemetry::default();
        m.error_flags = 0;
        m.motion_control_type = CcmMotionControlType(4);
        m.gain_stage_index = 2;
        m.set_current_telem(current_telem);
        m.set_velocity_telem(velocity_telem);
        m
    };
    let ext_low_current = {
        let mut e = ExtendedTelemetry::default();
        e.timestamp_us_lo = 5000000;
        e.set_power_status(nominal_power());
        e.set_front_left_motor(motor_zero_current());
        e.set_back_left_motor(motor_zero_current());
        e.set_back_right_motor(motor_zero_current());
        e.set_front_right_motor(motor_zero_current());
        e.set_body_control_telem(nominal_body_control());
        e.set_kicker_status(nominal_kicker());
        e
    };
    print_ext_row("Nominal, motors at rest (current samples = 0)", ext_low_current, raw_extended_telemetry);

    println!();
    println!("Note: proto size does NOT include framing overhead (e.g., the RadioHeader).");
    println!("      Raw size is always fixed regardless of field values.");
    println!("      Proto fields with default values (0/false) are NOT encoded.");
}

/// Encodes `msg` into a 2048-byte heapless buffer and asserts the byte count
/// matches `compute_size()`. Panics if they disagree (bug in our test data or micropb).
fn encode_and_validate<M>(msg: &M) -> usize
where
    M: MessageEncode + micropb::MessageDecode,
{
    let computed = msg.compute_size();
    let buf = heapless::Vec::<u8, 2048>::new();
    let mut enc = PbEncoder::new(buf);
    msg.encode(&mut enc).expect("encode overflowed 2048-byte buffer");
    let actual = enc.into_writer().len();
    assert_eq!(
        computed, actual,
        "compute_size()={computed} != actual encoded len={actual}"
    );
    actual
}

fn print_control_row(label: &str, msg: BasicControl, raw: usize) {
    let proto = encode_and_validate(&msg);
    print_row(label, proto, raw);
}

fn print_telemetry_row(label: &str, msg: BasicTelemetry, raw: usize) {
    let proto = encode_and_validate(&msg);
    print_row(label, proto, raw);
}

fn print_ext_row(label: &str, msg: ExtendedTelemetry, raw: usize) {
    let proto = encode_and_validate(&msg);
    print_row(label, proto, raw);
}

fn print_row(label: &str, proto: usize, raw: usize) {
    let diff = proto as i64 - raw as i64;
    let pct = (diff as f64 / raw as f64) * 100.0;
    let sign = if diff <= 0 { "saved" } else { "overhead" };
    println!(
        "  {:55} raw={:3}B  proto={:3}B  {:+4}B ({:.1}% {})",
        label, raw, proto, diff, pct.abs(), sign
    );
}
