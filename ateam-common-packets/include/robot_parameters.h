/**
 * @file robot_parameters.h
 * @author Will Stuckey
 * @brief packet definitions for parameter reads and write
 * @version 0.1
 * 
 * @copyright Copyright (c) 2023
 *
 */

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
    /// [COULOMB_FRICTION_COEFFICIENT_LINEAR, COULOMB_FRICTION_COEFFICIENT_ANGULAR, VISCOUS_FRICTION_COEFFICIENT_LINEAR, VISCOUS_FRICTION_COEFFICIENT_ANGULAR]
    PHYS_FRICTION_MODEL = 6,
    /// [FEEDFORWARD_GAIN, FEEDBACK_GAIN]
    POSE_CONTROL_GAIN = 7,
    /// [ERROR_POS_LINEAR, ERROR_POS_ANGULAR, ERROR_VEL_LINEAR, ERROR_VEL_ANGULAR] thresholds for when to recompute the trajectory
    TRAJ_RECOMPUTE_ERROR = 8,
    /// [MAX_VEL_LINEAR, MAX_VEL_ANGULAR, MAX_ACCEL_LINEAR, MAX_ACCEL_ANGULAR]
    TRAJ_MAX = 9,
    /// [P, I, D, I_MIN, I_MAX]
    POSE_FB_PIDII_X = 10,
    /// [P, I, D, I_MIN, I_MAX]
    POSE_FB_PIDII_Y = 11,
    /// [P, I, D, I_MIN, I_MAX]
    POSE_FB_PIDII_THETA = 12,
    /// [P, I, D, I_MIN, I_MAX]
    TWIST_FB_PIDII_X = 13,
    /// [P, I, D, I_MIN, I_MAX]
    TWIST_FB_PIDII_Y = 14,
    /// [P, I, D, I_MIN, I_MAX]
    TWIST_FB_PIDII_THETA = 15,
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
    } data __attribute__((aligned (4)));
} ParameterCommand;
assert_size(ParameterCommand, 24);