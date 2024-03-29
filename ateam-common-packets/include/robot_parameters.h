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
    VEL_PID_X = 0,
    VEL_PID_Y = 1,
    ANGULAR_VEL_PID_Z = 2,
    VEL_CGKF_ENCODER_NOISE = 3,
    VEL_CGKF_GYRO_NOISE = 4,
    VEL_CGKF_PROCESS_NOISE = 5,
    VEL_CGFK_INITIAL_COVARIANCE = 6,
    VEL_CGKF_K_MATRIX = 7,
    RC_BODY_VEL_LIMIT = 8,
    RC_BODY_ACC_LIMIT = 9,
    RC_WHEEL_ACC_LIMIT = 10,
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