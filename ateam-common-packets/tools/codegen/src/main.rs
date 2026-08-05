//! Codegen binary for ateam-common-packets.
//!
//! Run from the `ateam-common-packets/` root:
//!
//!   cargo run --manifest-path tools/codegen/Cargo.toml
//!
//! Writes committed C headers under `generated/include/`.
//! CI checks idempotency: `cargo run ... && git diff --exit-code generated/`.

use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

// ─── syn-based bitfield parsing ──────────────────────────────────────────────

struct BitField {
    name: String,
    bits: u32,
    reserved: bool,
    is_bool: bool,
    /// Rust return type for multi-bit fields (e.g. "u8")
    field_type: String,
}

struct BitfieldStruct {
    name: String,
    base_type: String, // "u8" | "u16" | "u32" | "u64"
    fields: Vec<BitField>,
}

fn c_uint(rust_type: &str) -> &str {
    match rust_type {
        "u8" => "uint8_t",
        "u16" => "uint16_t",
        "u32" => "uint32_t",
        "u64" => "uint64_t",
        _ => "uint32_t",
    }
}

fn bits_for_type(type_str: &str) -> u32 {
    match type_str {
        "bool" => 1,
        "u8" | "i8" => 8,
        "u16" | "i16" => 16,
        "u32" | "i32" => 32,
        "u64" | "i64" => 64,
        _ => 0,
    }
}

fn get_bits_attr(attrs: &[syn::Attribute]) -> Option<u32> {
    for attr in attrs {
        if attr.path().is_ident("bits") {
            if let Ok(n) = attr.parse_args::<syn::LitInt>() {
                return n.base10_parse().ok();
            }
        }
    }
    None
}

fn parse_bitfield_structs(src: &str) -> Vec<BitfieldStruct> {
    let file = match syn::parse_file(src) {
        Ok(f) => f,
        Err(_) => return vec![],
    };

    let mut out = Vec::new();

    for item in &file.items {
        let syn::Item::Struct(s) = item else { continue };

        // Look for #[wire_bitfield(uN)] attribute
        let mut base_type = None;
        for attr in &s.attrs {
            if attr.path().is_ident("wire_bitfield") {
                if let Ok(ident) = attr.parse_args::<syn::Ident>() {
                    base_type = Some(ident.to_string());
                }
            }
        }
        let Some(base_type) = base_type else { continue };

        let syn::Fields::Named(named) = &s.fields else { continue };

        let mut fields = Vec::new();
        for field in &named.named {
            let name = field.ident.as_ref().unwrap().to_string();
            let reserved = name.starts_with('_');

            let type_str = if let syn::Type::Path(tp) = &field.ty {
                tp.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default()
            } else {
                String::new()
            };

            let is_bool = type_str == "bool";
            let bits = get_bits_attr(&field.attrs).unwrap_or_else(|| bits_for_type(&type_str));

            fields.push(BitField {
                name,
                bits,
                reserved,
                is_bool,
                field_type: type_str,
            });
        }

        out.push(BitfieldStruct {
            name: s.ident.to_string(),
            base_type,
            fields,
        });
    }

    out
}

// ─── C accessor header generation ────────────────────────────────────────────

fn snake_to_prefix(name: &str) -> String {
    // BasicControlFlags -> bcf_, CcmTelemetryStatus -> cts_
    // Take the first letter of each CamelCase word.
    let mut prefix = String::new();
    let mut prev_upper = false;
    for ch in name.chars() {
        if ch.is_uppercase() {
            if prev_upper {
                // still in run of caps (e.g. "CCM") — skip
            } else {
                prefix.push(ch.to_lowercase().next().unwrap());
            }
            prev_upper = true;
        } else {
            prev_upper = false;
        }
    }
    prefix.push('_');
    prefix
}

fn generate_bitfield_header(bf: &BitfieldStruct) -> String {
    let c_type = c_uint(&bf.base_type);
    let prefix = snake_to_prefix(&bf.name);

    let mut out = String::new();
    let _ = writeln!(out, "#pragma once");
    let _ = writeln!(out, "#include <stdint.h>");
    let _ = writeln!(out, "#include <stdbool.h>");
    let _ = writeln!(out);
    let _ = writeln!(out, "typedef {c_type} {name};", c_type = c_type, name = bf.name);
    let _ = writeln!(out);

    let mut offset: u32 = 0;
    for field in &bf.fields {
        let off = offset;
        offset += field.bits;

        if field.reserved {
            continue;
        }

        let fname = &field.name;

        if field.is_bool {
            let mask_val = 1u64 << off;
            let _ = writeln!(out,
                "static inline bool {prefix}{fname}({name} v) {{ return (v & 0x{mask:X}u) != 0u; }}",
                prefix = prefix, fname = fname, name = bf.name, mask = mask_val);
            let _ = writeln!(out,
                "static inline {name} {prefix}with_{fname}({name} v, bool b) {{ return b ? (v | 0x{mask:X}u) : (v & ~0x{mask:X}u); }}",
                prefix = prefix, fname = fname, name = bf.name, mask = mask_val);
            let _ = writeln!(out,
                "static inline void {prefix}set_{fname}({name} *v, bool b) {{ *v = {prefix}with_{fname}(*v, b); }}",
                prefix = prefix, fname = fname, name = bf.name);
            let _ = writeln!(out);
        } else {
            let bits = field.bits;
            let mask_val: u64 = (1u64 << bits) - 1;
            let ret_type = c_uint(&field.field_type);
            let _ = writeln!(out,
                "static inline {ret} {prefix}{fname}({name} v) {{ return ({ret})((v >> {off}u) & 0x{mask:X}u); }}",
                ret = ret_type, prefix = prefix, fname = fname, name = bf.name,
                off = off, mask = mask_val);
            let _ = writeln!(out,
                "static inline {name} {prefix}with_{fname}({name} v, {ret} x) {{ return (v & ~(({name})(0x{mask:X}u) << {off}u)) | ((({name})(x) & 0x{mask:X}u) << {off}u); }}",
                ret = ret_type, prefix = prefix, fname = fname, name = bf.name,
                off = off, mask = mask_val);
            let _ = writeln!(out,
                "static inline void {prefix}set_{fname}({name} *v, {ret} x) {{ *v = {prefix}with_{fname}(*v, x); }}",
                ret = ret_type, prefix = prefix, fname = fname, name = bf.name);
            let _ = writeln!(out);
        }
    }

    let _ = writeln!(out);
    let _ = writeln!(out, "/* Total bits: {} */", offset);

    out
}

// ─── Main packets.h — hand-authored C from known type definitions ─────────────

fn generate_packets_h() -> String {
    // This is the authoritative C header for all wire packet types.
    // It is generated (not hand-written) so that CI can verify it is always
    // in sync with the Rust definitions in rust-lib/src/packets/.
    //
    // Layout is verified by the Rust `const _: () = assert!(size_of::<T>() == N)`
    // assertions in each packet module.  If a Rust size assertion fires, the
    // Rust crate won't compile; this header is only committed once it does.

    r#"// Auto-generated by tools/codegen — DO NOT EDIT.
// Source of truth: rust-lib/src/packets/ and rust-lib/src/bitfields/
// Run `cargo run --manifest-path tools/codegen/Cargo.toml` to regenerate.

#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#include "bitfields/BasicControlFlags.h"
#include "bitfields/BasicTelemetryErrors.h"
#include "bitfields/BodyControlExtTelemetryFlags.h"
#include "bitfields/BatteryStatus.h"
#include "bitfields/PowerStatus.h"
#include "bitfields/PowerCommandFlags.h"
#include "bitfields/DiscoveryFlags.h"
#include "bitfields/KickerCommandFlags.h"
#include "bitfields/KickerStatusFlags.h"
#include "bitfields/CcmMotionCommandFlags.h"
#include "bitfields/CcmTelemetryStatus.h"

// ─── Enums ───────────────────────────────────────────────────────────────────

typedef enum TeamColor {
    TeamColor_Yellow = 0,
    TeamColor_Blue   = 1,
} __attribute__((packed)) TeamColor;

typedef enum PivotDirection {
    PivotDirection_Forward  = 0,
    PivotDirection_Backward = 1,
} __attribute__((packed)) PivotDirection;

typedef enum KickRequest {
    KickRequest_Disable      = 0,
    KickRequest_Arm          = 1,
    KickRequest_KickNow      = 2,
    KickRequest_KickTouch    = 3,
    KickRequest_KickCaptured = 4,
    KickRequest_ChipNow      = 5,
    KickRequest_ChipTouch    = 6,
    KickRequest_ChipCaptured = 7,
} __attribute__((packed)) KickRequest;

typedef enum DribblerCommand {
    DribblerCommand_Disable     = 0,
    DribblerCommand_HardReceive = 1,
    DribblerCommand_SoftReceive = 2,
    DribblerCommand_Dribble     = 3,
    DribblerCommand_Velocity    = 4,
    DribblerCommand_Current     = 5,
} __attribute__((packed)) DribblerCommand;

typedef enum ParameterCommandCode {
    ParameterCommandCode_Read               = 0,
    ParameterCommandCode_Write              = 1,
    ParameterCommandCode_Ack                = 2,
    ParameterCommandCode_NackInvalidName    = 3,
    ParameterCommandCode_NackInvalidTypeForName = 4,
} __attribute__((packed)) ParameterCommandCode;

typedef enum ParameterName {
    ParameterName_KfProcessStd        = 0,
    ParameterName_KfMeasurementStd    = 1,
    ParameterName_KfMaxState          = 2,
    ParameterName_PhysWheel           = 3,
    ParameterName_PhysInertia         = 4,
    ParameterName_PhysMotorModel      = 5,
    ParameterName_PhysFrictionModel   = 6,
    ParameterName_FrictionCompGating  = 7,
    ParameterName_PoseControlGain     = 8,
    ParameterName_TrajRecomputeError  = 9,
    ParameterName_PoseFbPidiiLinear   = 10,
    ParameterName_PoseFbPidiiAngular  = 11,
    ParameterName_TwistFbPidiiLinear  = 12,
    ParameterName_TwistFbPidiiAngular = 13,
} __attribute__((packed)) ParameterName;

typedef enum CcmMotionControlType {
    CcmMotionControlType_MotorOff        = 0,
    CcmMotionControlType_DutyOpenloop    = 1,
    CcmMotionControlType_VoltageOpenloop = 2,
    CcmMotionControlType_Current         = 3,
    CcmMotionControlType_Velocity        = 4,
    CcmMotionControlType_VelocityCurrent = 5,
} __attribute__((packed)) CcmMotionControlType;

// ─── Leaf maneuver structs ───────────────────────────────────────────────────

typedef struct GlobalPositionCommand {
    float global_x;
    float global_y;
    float global_theta;
    float max_linear_vel;
    float max_angular_vel;
    float max_linear_acc;
    float max_angular_acc;
} GlobalPositionCommand;

typedef struct GlobalPositionTelemetry { uint32_t reserved; } GlobalPositionTelemetry;
typedef struct ExtendedGlobalPositionTelemetry { GlobalPositionCommand cmd_echo; } ExtendedGlobalPositionTelemetry;

typedef struct GlobalVelocityCommand {
    float global_xd;
    float global_yd;
    float global_omega;
    float max_linear_acc;
    float max_angular_acc;
} GlobalVelocityCommand;

typedef struct GlobalVelocityTelemetry { uint32_t reserved; } GlobalVelocityTelemetry;
typedef struct ExtendedGlobalVelocityTelemetry { GlobalVelocityCommand cmd_echo; } ExtendedGlobalVelocityTelemetry;

typedef struct LocalVelocityCommand {
    float local_xd;
    float local_yd;
    float local_omega;
    float max_linear_acc;
    float max_angular_acc;
} LocalVelocityCommand;

typedef struct LocalVelocityTelemetry { uint32_t reserved; } LocalVelocityTelemetry;
typedef struct ExtendedLocalVelocityTelemetry { LocalVelocityCommand cmd_echo; } ExtendedLocalVelocityTelemetry;

typedef struct GlobalAccelerationCommand { float global_xdd; float global_ydd; float global_alpha; } GlobalAccelerationCommand;
typedef struct GlobalAccelerationTelemetry { uint32_t reserved; } GlobalAccelerationTelemetry;
typedef struct ExtendedGlobalAccelerationTelemetry { GlobalAccelerationCommand cmd_echo; } ExtendedGlobalAccelerationTelemetry;

typedef struct LocalAccelerationCommand { float local_xdd; float local_ydd; float local_alpha; } LocalAccelerationCommand;
typedef struct LocalAccelerationTelemetry { uint32_t reserved; } LocalAccelerationTelemetry;
typedef struct ExtendedLocalAccelerationTelemetry { LocalAccelerationCommand cmd_echo; } ExtendedLocalAccelerationTelemetry;

typedef struct HeadingPivotCommand {
    float global_theta;
    float max_angular_vel;
    float max_angular_acc;
    float orbit_radius;
    float inset_angle;
    PivotDirection direction;
    uint8_t compute_inset_angle;
    uint8_t reserved[2];
} HeadingPivotCommand;

typedef struct PointPivotCommand {
    float target_x;
    float target_y;
    float max_angular_vel;
    float max_angular_acc;
    float orbit_radius;
    float inset_angle;
    PivotDirection direction;
    uint8_t compute_inset_angle;
    uint8_t reserved[2];
} PointPivotCommand;

typedef struct HeadingPivotTelemetry { uint32_t reserved; } HeadingPivotTelemetry;
typedef struct PointPivotTelemetry { uint32_t reserved; } PointPivotTelemetry;
typedef struct ExtendedHeadingPivotTelemetry { HeadingPivotCommand cmd_echo; } ExtendedHeadingPivotTelemetry;
typedef struct ExtendedPointPivotTelemetry { PointPivotCommand cmd_echo; } ExtendedPointPivotTelemetry;

typedef struct HeadingLineCommand {
    float start_x; float start_y; float dir_x; float dir_y; float line_velocity;
    float global_theta; float max_vel_colinear; float max_vel_perp; float max_vel_angular;
    float max_accel_colinear; float max_accel_perp; float max_accel_angular;
    float colinear_start_thresh;
} HeadingLineCommand;

typedef struct PointLineCommand {
    float start_x; float start_y; float dir_x; float dir_y; float line_velocity;
    float target_x; float target_y;
    float max_vel_colinear; float max_vel_perp; float max_vel_angular;
    float max_accel_colinear; float max_accel_perp; float max_accel_angular;
    float colinear_start_thresh;
} PointLineCommand;

typedef struct HeadingLineTelemetry { uint32_t reserved; } HeadingLineTelemetry;
typedef struct PointLineTelemetry { uint32_t reserved; } PointLineTelemetry;
typedef struct ExtendedHeadingLineTelemetry { HeadingLineCommand cmd_echo; } ExtendedHeadingLineTelemetry;
typedef struct ExtendedPointLineTelemetry { PointLineCommand cmd_echo; } ExtendedPointLineTelemetry;

// ─── Discovery ───────────────────────────────────────────────────────────────

typedef struct HelloRequest {
    uint8_t robot_id;
    TeamColor color;
    DiscoveryFlags flags;
    uint8_t _reserved[1];
    uint8_t coms_hash[4];
    uint8_t controls_hash[4];
    uint8_t firmware_hash[4];
} HelloRequest;

typedef struct HelloResponse { uint8_t ipv4[4]; uint16_t port; } HelloResponse;

// ─── Error telemetry ─────────────────────────────────────────────────────────

typedef struct ErrorTelemetry { uint32_t timestamp; uint8_t error_message[60]; } ErrorTelemetry;

// ─── Power ───────────────────────────────────────────────────────────────────

typedef struct BatteryInfo {
    BatteryStatus status;
    uint16_t battery_mv;
    uint16_t cell1_mv; uint16_t cell2_mv; uint16_t cell3_mv;
    uint16_t cell4_mv; uint16_t cell5_mv; uint16_t cell6_mv;
    uint8_t battery_pct;
    uint8_t cell1_pct; uint8_t cell2_pct; uint8_t cell3_pct;
    uint8_t cell4_pct; uint8_t cell5_pct; uint8_t cell6_pct;
    uint8_t _reserved;
} BatteryInfo;

typedef struct PowerTelemetry { PowerStatus status; BatteryInfo battery_info; } PowerTelemetry;
typedef struct PowerCommand { PowerCommandFlags flags; } PowerCommand;

// ─── CCM (Current-Controlled Motor) ─────────────────────────────────────────

typedef struct CcmMotionCommand {
    CcmMotionCommandFlags flags;
    CcmMotionControlType motion_control_type;
    uint8_t _reserved[1];
    int16_t current_setpoint_ma;
    float setpoint;
} CcmMotionCommand;

typedef struct CcmCurrentTelemetry {
    uint16_t bus_voltage_mv;
    uint16_t motor_voltage_cmd_mv;
    int16_t current_setpoint_ma;
    int16_t hall_vel_est_drads;
    uint16_t current_samples_ma[20];
} CcmCurrentTelemetry;

typedef struct CcmVelocityTelemetry { float vel_setpoint_rads; float wheel_vel_rads; } CcmVelocityTelemetry;

typedef struct CcmTelemetry {
    CcmTelemetryStatus status;
    CcmMotionControlType motion_control_type;
    uint8_t gain_stage_index;
    CcmCurrentTelemetry current_telemetry;
    CcmVelocityTelemetry velocity_telemetry;
} CcmTelemetry;

// ─── Kicker ──────────────────────────────────────────────────────────────────

typedef struct KickerControl {
    KickerCommandFlags flags;
    DribblerCommand dribbler_mode;
    KickRequest kick_request;
    float kick_speed;
    float drib_setpoint;
} KickerControl;

typedef struct KickerTelemetry {
    KickerStatusFlags status;
    uint16_t charge_pct;
    float rail_voltage;
    float battery_voltage;
    uint8_t kicker_image_hash[4];
    CcmTelemetry dribbler_motor;
} KickerTelemetry;

// ─── BodyControlCommand (discriminated union) ─────────────────────────────────
// cbindgen-style: explicit Tag enum + Body union + outer struct.
// Python generate_msgs.py detects this by the `tag` + `body` field pattern.

typedef enum BodyControlCommandTag {
    BodyControlCommandTag_Off               = 0,
    BodyControlCommandTag_EstopBrake        = 1,
    BodyControlCommandTag_GlobalPosition    = 10,
    BodyControlCommandTag_GlobalVelocity    = 11,
    BodyControlCommandTag_LocalVelocity     = 12,
    BodyControlCommandTag_GlobalAcceleration = 13,
    BodyControlCommandTag_LocalAcceleration = 14,
    BodyControlCommandTag_HeadingPivot      = 20,
    BodyControlCommandTag_PointPivot        = 21,
    BodyControlCommandTag_HeadingLine       = 30,
    BodyControlCommandTag_PointLine         = 31,
} __attribute__((packed)) BodyControlCommandTag;

typedef union BodyControlCommandBody {
    GlobalPositionCommand    global_position;
    GlobalVelocityCommand    global_velocity;
    LocalVelocityCommand     local_velocity;
    GlobalAccelerationCommand global_acceleration;
    LocalAccelerationCommand local_acceleration;
    HeadingPivotCommand      heading_pivot;
    PointPivotCommand        point_pivot;
    HeadingLineCommand       heading_line;
    PointLineCommand         point_line;
} BodyControlCommandBody;

typedef struct BodyControlCommand {
    BodyControlCommandTag tag;
    BodyControlCommandBody body;
} BodyControlCommand;

// ─── BasicControl (88B) ──────────────────────────────────────────────────────

typedef struct BasicControl {
    BasicControlFlags flags;
    float vision_position_update[3];
    KickRequest kick_request;
    uint8_t play_song;
    DribblerCommand dribbler_mode;
    uint8_t _pad;
    float kick_vel;
    float dribbler_setpoint;
    BodyControlCommand cmd;
} BasicControl;

// ─── BodyControlTelemetry (discriminated union) ───────────────────────────────

typedef enum BodyControlTelemetryTag {
    BodyControlTelemetryTag_Off               = 0,
    BodyControlTelemetryTag_EstopBrake        = 1,
    BodyControlTelemetryTag_GlobalPosition    = 10,
    BodyControlTelemetryTag_GlobalVelocity    = 11,
    BodyControlTelemetryTag_LocalVelocity     = 12,
    BodyControlTelemetryTag_GlobalAcceleration = 13,
    BodyControlTelemetryTag_LocalAcceleration = 14,
    BodyControlTelemetryTag_HeadingPivot      = 20,
    BodyControlTelemetryTag_PointPivot        = 21,
    BodyControlTelemetryTag_HeadingLine       = 30,
    BodyControlTelemetryTag_PointLine         = 31,
} __attribute__((packed)) BodyControlTelemetryTag;

typedef union BodyControlTelemetryBody {
    GlobalPositionTelemetry    global_position;
    GlobalVelocityTelemetry    global_velocity;
    LocalVelocityTelemetry     local_velocity;
    GlobalAccelerationTelemetry global_acceleration;
    LocalAccelerationTelemetry local_acceleration;
    HeadingPivotTelemetry      heading_pivot;
    PointPivotTelemetry        point_pivot;
    HeadingLineTelemetry       heading_line;
    PointLineTelemetry         point_line;
} BodyControlTelemetryBody;

typedef struct BodyControlTelemetry {
    BodyControlTelemetryTag tag;
    BodyControlTelemetryBody body;
} BodyControlTelemetry;

// ─── BasicTelemetry (32B) ────────────────────────────────────────────────────

typedef struct BasicTelemetry {
    uint8_t transmission_sequence_number;
    uint8_t control_data_sequence_number;
    uint8_t _reserved[2];
    BasicTelemetryErrors errors;
    uint16_t battery_percent;
    uint16_t kicker_charge_percent;
    BodyControlTelemetry control_telem;
    int16_t kf_body_pos_estimate[3];
    int16_t kf_body_vel_estimate[3];
} BasicTelemetry;

// ─── BodyControlManeuverExtendedTelemetry (discriminated union) ───────────────

typedef enum BodyControlManeuverExtendedTelemetryTag {
    BodyControlManeuverExtendedTelemetryTag_Off               = 0,
    BodyControlManeuverExtendedTelemetryTag_EstopBrake        = 1,
    BodyControlManeuverExtendedTelemetryTag_GlobalPosition    = 10,
    BodyControlManeuverExtendedTelemetryTag_GlobalVelocity    = 11,
    BodyControlManeuverExtendedTelemetryTag_LocalVelocity     = 12,
    BodyControlManeuverExtendedTelemetryTag_GlobalAcceleration = 13,
    BodyControlManeuverExtendedTelemetryTag_LocalAcceleration = 14,
    BodyControlManeuverExtendedTelemetryTag_HeadingPivot      = 20,
    BodyControlManeuverExtendedTelemetryTag_PointPivot        = 21,
    BodyControlManeuverExtendedTelemetryTag_HeadingLine       = 30,
    BodyControlManeuverExtendedTelemetryTag_PointLine         = 31,
} __attribute__((packed)) BodyControlManeuverExtendedTelemetryTag;

typedef union BodyControlManeuverExtendedTelemetryBody {
    ExtendedGlobalPositionTelemetry    global_position;
    ExtendedGlobalVelocityTelemetry    global_velocity;
    ExtendedLocalVelocityTelemetry     local_velocity;
    ExtendedGlobalAccelerationTelemetry global_acceleration;
    ExtendedLocalAccelerationTelemetry local_acceleration;
    ExtendedHeadingPivotTelemetry      heading_pivot;
    ExtendedPointPivotTelemetry        point_pivot;
    ExtendedHeadingLineTelemetry       heading_line;
    ExtendedPointLineTelemetry         point_line;
} BodyControlManeuverExtendedTelemetryBody;

typedef struct BodyControlManeuverExtendedTelemetry {
    BodyControlManeuverExtendedTelemetryTag tag;
    BodyControlManeuverExtendedTelemetryBody body;
} BodyControlManeuverExtendedTelemetry;

// ─── BodyControlExtendedTelemetry (208B) ─────────────────────────────────────

typedef struct BodyControlExtendedTelemetry {
    BodyControlExtTelemetryFlags flags;
    uint8_t _reserved[3];
    BodyControlManeuverExtendedTelemetry maneuver;
    float imu_gyro[3];
    float imu_accel[3];
    float vision_pose[3];
    float body_traj_pos[3];
    float body_traj_vel[3];
    float kf_body_pos_prediction[3];
    float kf_body_vel_prediction[3];
    float kf_body_pos_estimate[3];
    float kf_body_vel_estimate[3];
    float body_vel_u[3];
    float body_accel_u[3];
    float body_accel_u_fric_comp[3];
} BodyControlExtendedTelemetry;

// ─── ParameterData (discriminated union) ─────────────────────────────────────

typedef enum ParameterDataTag {
    ParameterDataTag_F32     = 0,
    ParameterDataTag_Vec2F32 = 1,
    ParameterDataTag_Vec3F32 = 2,
    ParameterDataTag_Vec4F32 = 3,
    ParameterDataTag_Vec5F32 = 4,
    ParameterDataTag_Vec6F32 = 5,
} __attribute__((packed)) ParameterDataTag;

typedef union ParameterDataBody {
    float f32[1];
    float vec2_f32[2];
    float vec3_f32[3];
    float vec4_f32[4];
    float vec5_f32[5];
    float vec6_f32[6];
} ParameterDataBody;

typedef struct ParameterData {
    ParameterDataTag tag;
    ParameterDataBody body;
} ParameterData;

typedef struct ParameterCommand {
    ParameterCommandCode command_code;
    ParameterName parameter_name;
    uint8_t _pad[2];
    ParameterData data;
} ParameterCommand;

// ─── ExtendedTelemetry (560B) ────────────────────────────────────────────────

typedef struct ExtendedTelemetry {
    uint32_t timestamp_us_lo;
    uint32_t timestamp_us_hi;
    PowerTelemetry power_status;
    CcmTelemetry front_left_motor;
    CcmTelemetry back_left_motor;
    CcmTelemetry back_right_motor;
    CcmTelemetry front_right_motor;
    BodyControlExtendedTelemetry body_control_telemetry;
    KickerTelemetry kicker_status;
} ExtendedTelemetry;

// ─── RadioData (discriminated union) ─────────────────────────────────────────

typedef enum RadioDataTag {
    RadioDataTag_HelloRequest           = 21,
    RadioDataTag_HelloResponse          = 22,
    RadioDataTag_Telemetry              = 41,
    RadioDataTag_ControlDebugTelemetry  = 42,
    RadioDataTag_RobotParameterCommand  = 43,
    RadioDataTag_ErrorTelemetry         = 44,
    RadioDataTag_Control                = 61,
} __attribute__((packed)) RadioDataTag;

typedef union RadioDataBody {
    HelloRequest        hello_request;
    HelloResponse       hello_response;
    BasicTelemetry      telemetry;
    ExtendedTelemetry   control_debug_telemetry;
    ParameterCommand    robot_parameter_command;
    ErrorTelemetry      error_telemetry;
    BasicControl        control;
} RadioDataBody;

typedef struct RadioData {
    RadioDataTag tag;
    RadioDataBody body;
} RadioData;

// ─── RadioHeader & RadioPacket (572B, ≤576B) ─────────────────────────────────

typedef struct __attribute__((packed)) RadioHeader {
    uint32_t crc32;
    uint8_t _reserved[2];
    uint16_t data_length;
} RadioHeader;

typedef struct __attribute__((aligned(4))) RadioPacket {
    RadioHeader header;
    RadioData data;
} RadioPacket;

static_assert(sizeof(RadioPacket) <= 576, "RadioPacket exceeds 576-byte buffer limit");
"#.to_string()
}

// ─── Entry point ─────────────────────────────────────────────────────────────

fn main() {
    // Expect to run from the ateam-common-packets root.
    let root = Path::new(".");
    let bitfields_src = root.join("rust-lib/src/bitfields/mod.rs");
    let out_dir = root.join("generated/include");
    let bitfields_out_dir = out_dir.join("bitfields");

    fs::create_dir_all(&bitfields_out_dir).expect("create generated/include/bitfields/");

    // --- Parse bitfield sources ---
    let src = fs::read_to_string(&bitfields_src)
        .unwrap_or_else(|e| panic!("reading {}: {e}", bitfields_src.display()));

    let bitfield_structs = parse_bitfield_structs(&src);

    if bitfield_structs.is_empty() {
        eprintln!("WARNING: no #[wire_bitfield] structs found in {}", bitfields_src.display());
    }

    // --- Write per-type bitfield headers ---
    for bf in &bitfield_structs {
        let header = generate_bitfield_header(bf);
        let path = bitfields_out_dir.join(format!("{}.h", bf.name));
        let existing = fs::read_to_string(&path).unwrap_or_default();
        if existing != header {
            fs::write(&path, &header)
                .unwrap_or_else(|e| panic!("writing {}: {e}", path.display()));
            eprintln!("wrote {}", path.display());
        }
    }

    // --- Write packets.h ---
    let packets_h = generate_packets_h();
    let packets_path = out_dir.join("packets.h");
    let existing = fs::read_to_string(&packets_path).unwrap_or_default();
    if existing != packets_h {
        fs::write(&packets_path, &packets_h)
            .unwrap_or_else(|e| panic!("writing {}: {e}", packets_path.display()));
        eprintln!("wrote {}", packets_path.display());
    }

    // --- Write top-level umbrella header ---
    let umbrella = "// Auto-generated by tools/codegen — DO NOT EDIT.\n\
                    #pragma once\n\
                    #include \"packets.h\"\n";
    let umbrella_path = out_dir.join("ateam_packets.h");
    let existing = fs::read_to_string(&umbrella_path).unwrap_or_default();
    if existing != umbrella {
        fs::write(&umbrella_path, umbrella)
            .unwrap_or_else(|e| panic!("writing {}: {e}", umbrella_path.display()));
        eprintln!("wrote {}", umbrella_path.display());
    }

    eprintln!("codegen complete.");
}
