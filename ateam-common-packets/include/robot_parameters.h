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
    /// [MOTOR_TORQUE_CONSTANT, MOTOR_EFFICIENCY_FACTOR]
    PHYS_MOTOR_MODEL = 5,
    /// [COULOMB_LINEAR, COULOMB_ANGULAR, VISCOUS_LINEAR, VISCOUS_ANGULAR]; linear terms are for motion along local +x and scaled by a wheel-geometry direction factor otherwise
    PHYS_FRICTION_MODEL = 6,
    /// [LINEAR_VEL_THRESHOLD, ANGULAR_VEL_THRESHOLD] commanded speeds below which friction compensation is off
    FRICTION_COMP_VEL_THRESHOLD = 7,
    /// [FEEDFORWARD_GAIN, FEEDBACK_GAIN] trajectory tracking accel (torque) path
    TRACKING_ACCEL_GAIN = 8,
    /// [ERROR_POS_LINEAR, ERROR_POS_ANGULAR, ERROR_VEL_LINEAR, ERROR_VEL_ANGULAR] trajectory tracking divergence thresholds
    TRACKING_DIVERGENCE_THRESHOLD = 9,
    /// [P, I, D, I_MIN, I_MAX]
    POSE_FB_PIDII_LINEAR = 10,
    /// [P, I, D, I_MIN, I_MAX]
    POSE_FB_PIDII_ANGULAR = 11,
    /// [P, I, D, I_MIN, I_MAX]
    TWIST_FB_PIDII_LINEAR = 12,
    /// [P, I, D, I_MIN, I_MAX]
    TWIST_FB_PIDII_ANGULAR = 13,
    /// [SEED_SAMPLES] consistent vision samples needed to (re)seed the vision filter
    VISION_SEED_SAMPLES = 14,
    /// [STATIC_LINEAR, STATIC_ANGULAR, STRIBECK_VEL_LINEAR, STRIBECK_VEL_ANGULAR]; static friction at zero speed decaying to coulomb over the Stribeck velocity (disabled if vel <= 0 or static <= coulomb)
    PHYS_STRIBECK_MODEL = 15,
    /// [CUTOFF_LINEAR_HZ, CUTOFF_ANGULAR_HZ] first-order low-pass on the trajectory tracking feedforward accel (disabled if <= 0)
    TRACKING_FF_ACCEL_CUTOFF_HZ = 16,
    /// [ERROR_LINEAR, ERROR_ANGULAR] trajectory-vs-estimate pose error above which breakaway friction comp is applied toward the target while the commanded speed is below FRICTION_COMP_VEL_THRESHOLD (disabled if <= 0)
    FRICTION_COMP_HOLD_ERR_THRESHOLD = 17,
    /// [LOOKAHEAD_S] trajectory velocity lookahead for the tracking body velocity (wheel velocity) output
    TRACKING_VEL_LOOKAHEAD_S = 18,
    /// [HOLD_VEL_LOOP_OFF, KV_LINEAR, KV_ANGULAR] wheel velocity loop off while the tracking trajectory holds (if != 0); pose feedback gains added to the wheel velocity setpoint (0 disables)
    TRACKING_WHEEL_VEL_FB = 19,
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