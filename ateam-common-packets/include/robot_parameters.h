#pragma once

#include "common.h"

typedef enum ParameterCommandCode : uint8_t {
    PCC_READ = 0,
    PCC_WRITE = 1,
    PCC_ACK = 2,
    PCC_NACK_INVALID_NAME = 3,
    PCC_NACK_INVALID_TYPE_FOR_NAME = 4,
} ParameterCommandCode;
assert_size(ParameterCommandCode, 1);

typedef enum ParameterDataFormat : uint8_t {
    F32 = 0,
    VEC2_F32 = 1,
    VEC3_F32 = 2,
    VEC4_F32 = 3,
    VEC5_F32 = 4,
    VEC6_F32 = 5,
} ParameterDataFormat;
assert_size(ParameterDataFormat, 1);

typedef enum ParameterName : uint8_t {
    /// [POS_LINEAR, POS_ANGULAR, VEL_LINEAR, VEL_ANGULAR]
    KF_PROCESS_STD = 0,
    /// [VISION_STD_LINEAR, VISION_STD_ANGULAR, ENCODER_STD_ANGULAR, GYRO_STD_ANGULAR]
    KF_MEASUREMENT_STD = 1,
    /// [KF_MAX_POS_LINEAR, KF_MAX_POS_ANGULAR, KF_MAX_VEL_LINEAR, KF_MAX_VEL_ANGULAR]
    KF_MAX_STATE = 2,
    /// [WHEEL_ANGLE_ALPHA, WHEEL_ANGLE_BETA, WHEEL_DISTANCE, WHEEL_RADIUS]
    PHYS_WHEEL = 3,
    /// [BODY_MASS, BODY_MOMENT_Z]
    PHYS_INERTIA = 4,
    /// [MOTOR_TORQUE_CONSTANT]
    PHYS_MOTOR_MODEL = 5,
    /// [COULOMB_XY, VISCOUS_XY, COULOMB_W, VISCOUS_W, EFFICIENCY_XY, EFFICIENCY_W]
    /// TIGERs Mannheim friction/output model; linear terms are for sideways (local y)
    /// motion and scaled by a wheel-geometry direction factor otherwise
    PHYS_FRICTION_MODEL = 6,
    // 7 retired (was FRICTION_COMP_VEL_THRESHOLD)
    // 8 retired (was TRACKING_ACCEL_GAIN)
    /// [ERROR_POS_LINEAR, ERROR_POS_ANGULAR, ERROR_VEL_LINEAR, ERROR_VEL_ANGULAR] trajectory tracking divergence thresholds
    TRACKING_DIVERGENCE_THRESHOLD = 9,
    // 10-13 retired (were POSE_FB_PIDII_*, TWIST_FB_PIDII_*)
    /// [SEED_SAMPLES] consistent vision samples needed to (re)seed the vision filter
    VISION_SEED_SAMPLES = 14,
    /// [K (1/s), MAX (m/s), ANTI_JITTER (m)] XY position P feedback into the velocity setpoint
    POSE_FB_XY = 15,
    /// [POS_K (1/s), POS_MAX (rad/s), ANTI_JITTER (rad), VEL_K] angular position + velocity P feedback into the velocity setpoint
    POSE_FB_W = 16,
    /// [KP (mA/(rad/s)), KI (mA/rad), ANTI_JITTER (rad/s), MAX_CURRENT (A)] wheel velocity loop on the
    /// motor controllers; MAX_CURRENT caps the total (feed-forward + loop) wheel current
    WHEEL_VEL_FB = 17,
    /// [MAX_CURRENT (A)] feed-forward wheel current limit; the body accel output is scaled to stay under it
    WHEEL_FF_CURRENT_LIMIT = 18,
    /// [K ((rad/s)/A)] wheel slip feed-forward: each wheel's velocity setpoint is raised by
    /// K * its feed-forward current, since the wheel surface outruns the ground under load
    WHEEL_SLIP_COMP = 19,
    /// [MIN_LATENCY (ms)] minimum vision latency, camera exposure -> robot; sets the
    /// robot/vision clock skew estimate (per field setup, see calibration/vision_latency.py)
    VISION_MIN_LATENCY = 20,
    /// [ACTION] read: 1 if a stored per-robot/per-setup config was applied at boot, else 0.
    /// Write 1: save the live PHYS_FRICTION_MODEL, PHYS_INERTIA, WHEEL_SLIP_COMP,
    /// VISION_MIN_LATENCY and KF_MEASUREMENT_STD (vision) to flash; write 2: clear it.
    /// Writes need the motors off and reboot the robot (flash is only written at boot).
    PERSIST_CONFIG = 21,
} ParameterName;
assert_size(ParameterName, 1);

typedef struct ParameterCommand {
    ParameterCommandCode command_code;
    ParameterDataFormat data_format;
    ParameterName parameter_name;
    union ParameterData {
        float f32;
        float vec2_f32[2];
        float vec3_f32[3];
        float vec4_f32[4];
        float vec5_f32[5];
        float vec6_f32[6];
    } data __attribute__((aligned (4)));
} ParameterCommand;
assert_size(ParameterCommand, 28);