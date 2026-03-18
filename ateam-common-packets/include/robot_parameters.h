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
    VEC3_F32 = 1,
    VEC4_F32 = 2,
    PID_F32 = 3,
    PID_LIMITED_INTEGRAL_F32 = 4,
    MATRIX_F32 = 5
} ParameterDataFormat;
assert_size(ParameterDataFormat, 1);

typedef enum ParameterName : uint8_t {
    KF_PROCESS_STD_POS_LINEAR = 0,
    KF_PROCESS_STD_POS_ANGULAR = 1,
    KF_PROCESS_STD_VEL_LINEAR = 2,
    KF_PROCESS_STD_VEL_ANGULAR = 3,
    KF_VISION_STD_LINEAR = 4,
    KF_VISION_STD_ANGULAR = 5,
    KF_ENCODER_STD_ANGULAR = 6,
    KF_GYRO_STD_ANGULAR = 7,
    KF_MAX_POS_LINEAR = 8,
    KF_MAX_POS_ANGULAR = 9,
    KF_MAX_VEL_LINEAR = 10,
    KF_MAX_VEL_ANGULAR = 11,
    PHYS_WHEEL_ANGLE_ALPHA = 12,
    PHYS_WHEEL_ANGLE_BETA = 13,
    PHYS_WHEEL_DISTANCE = 14,
    PHYS_WHEEL_RADIUS = 15,
    PHYS_BODY_MASS = 16,
    PHYS_BODY_MOMENT_Z = 17,
    PHYS_MOTOR_TORQUE_CONSTANT = 18,
    PHYS_MOTOR_EFFICIENCY_FACTOR = 19,
    PHYS_COULOMB_FRICTION_COEFFICIENT_LINEAR = 20,
    PHYS_COULOMB_FRICTION_COEFFICIENT_ANGULAR = 21,
    PHYS_VISCOUS_FRICTION_COEFFICIENT_LINEAR = 22,
    PHYS_VISCOUS_FRICTION_COEFFICIENT_ANGULAR = 23,
    PIDII_X = 24,
    PIDII_Y = 25,
    PIDII_THETA = 26,
    PIDII_XD = 27,
    PIDII_YD = 28,
    PIDII_THETAD = 29,
    TRAJ_ALLOWABLE_ERROR_POS_LINEAR = 30,
    TRAJ_ALLOWABLE_ERROR_POS_ANGULAR = 31,
    TRAJ_ALLOWABLE_ERROR_VEL_LINEAR = 32,
    TRAJ_ALLOWABLE_ERROR_VEL_ANGULAR = 33,
    TRAJ_MAX_VEL_LINEAR = 34,
    TRAJ_MAX_VEL_ANGULAR = 35,
    TRAJ_MAX_ACCEL_LINEAR = 36,
    TRAJ_MAX_ACCEL_ANGULAR = 37,
    HYST_PID_ENTER_ERROR_POS_LINEAR = 38,
    HYST_PID_ENTER_ERROR_POS_ANGULAR = 39,
    HYST_PID_ENTER_ERROR_VEL_LINEAR = 40,
    HYST_PID_ENTER_ERROR_VEL_ANGULAR = 41,
    HYST_PID_EXIT_ERROR_POS_LINEAR = 42,
    HYST_PID_EXIT_ERROR_POS_ANGULAR = 43,
    HYST_PID_EXIT_ERROR_VEL_LINEAR = 44,
    HYST_PID_EXIT_ERROR_VEL_ANGULAR = 45,
    TRAJ_RECOMPUTE_ERROR_POS_LINEAR = 46,
    TRAJ_RECOMPUTE_ERROR_VEL_LINEAR = 47,
    TRAJ_RECOMPUTE_ERROR_POS_ANGULAR = 48,
    TRAJ_RECOMPUTE_ERROR_VEL_ANGULAR = 49,
    FEEDFORWARD_GAIN = 50,
    FEEDBACK_GAIN = 51,
} ParameterName;
assert_size(ParameterName, 1);

typedef struct ParameterCommand {
    ParameterCommandCode command_code;
    ParameterDataFormat data_format;
    ParameterName parameter_name;
    union ParameterData {
        float f32;
        float vec3_f32[3];
        float vec4_f32[4];
        float pid_f32[3];
        float pidii_f32[5];
        float matrix_f32[25];
    } data __attribute__((aligned (4)));
} ParameterCommand;
assert_size(ParameterCommand, 104);