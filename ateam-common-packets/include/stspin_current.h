/**
 * @file stspin.h
 * @author Will Stuckey
 * @brief communication packet definitions for the stspin firmware
 * @version 0.1
 *
 * @copyright Copyright (c) 2022
 *
 * Struct sizes should pass static checks on the following 3 platforms:
 *  - Embedded -> C11, 4 byte ptr
 *  - Bindgen -> C11, 8 byte ptr
 *  - ROS -> C++, 8 byte ptr
 *
 */

#pragma once

#include "common.h"

typedef enum CcmCommandType {
    CCM_CMD_PARAMS = 0x20,
    CCM_CMD_MOTION = 0x21
} __attribute__((packed)) CcmCommandType;
assert_size(CcmCommandType, 1);

typedef enum CcmParameter {
    CCM_PARAM_CURRENT_PI_CONTROLLER_GAIN_P = 0,
    CCM_PARAM_CURRENT_PI_CONTROLLER_GAIN_I = 1,
    CCM_PARAM_CURRENT_PI_CONTROLLER_ERR_ILIM = 2,
    CCM_PARAM_CURRENT_PI_CONTROLLER_ANTIJITTER = 3,
    CCM_PARAM_FIRMWARE_IMAGE_HASH = 100,
} __attribute__((packed)) CcmParameter;
assert_size(CcmParameter, 1);

typedef enum CcmParameterOperation {
    CCM_PARAMOP_READ = 0,
    CCM_PARAMOP_WRITE = 1,
} __attribute__((packed)) CcmParameterOperation;
assert_size(CcmParameterOperation, 1);

typedef enum CcmParameterDirection {
    CCM_PARAMDIR_COMMAND = 0,
    CCM_PARAMDIR_REPLY = 1,
} __attribute__((packed)) CcmParameterDirection;
assert_size(CcmParameterDirection, 1);

typedef union CcmParameterValue {
    uint32_t val_u32;
    int32_t val_i32;
    float val_f32;
    uint8_t val_u8x4[4];
} CcmParameterValue;

typedef struct CurrentControlledMotor_ParameterPacket {
    CcmParameter parameter;
    CcmParameterOperation parameter_operation;
    CcmParameterDirection parameter_direction;
    uint8_t _reserved[1];
    CcmParameterValue value;
} __attribute__((packed)) CcmParameterPacket;
assert_size(CcmParameterPacket, 8);

typedef enum CcmMotionControlType {
    CCM_MCT_MOTOR_OFF = 0,
    CCM_MCT_DUTY_OPENLOOP = 1,
    CCM_MCT_VOLTAGE_OPENLOOP = 2,
    CCM_MCT_CURRENT = 3,
    CCM_MCT_VELOCITY = 4,
    CCM_MCT_VELOCITY_CURRENT = 5
} __attribute__((packed)) CcmMotionControlType;
assert_size(CcmMotionControlType, 1);

typedef struct CcmMotionCommand {
    uint32_t reset : 1;
    uint32_t enable_telemetry: 1;
    uint32_t enable_motion: 1;
    uint32_t calibrate_current: 1;
    uint32_t _reserved : 28;

    CcmMotionControlType motion_control_type;
    uint8_t _reserved_2[1];
    int16_t current_setpoint_ma;

    float setpoint;
} __attribute__((__packed__)) CcmMotionCommand;
assert_size(CcmMotionCommand, 12); // Note: Same length as MotorCommand_Params_Packet

typedef struct CcmCommand {
    CcmCommandType type;
    uint8_t _reserved[3];
    uint32_t crc32;
    union CcmCommandData {
        CcmParameterPacket param;
        CcmMotionCommand motion;
    } data;
} __attribute__((__packed__)) CcmCommand;
assert_size(CcmCommand, 20);

/////////////////
//  responses  //
/////////////////

typedef enum CcmResponseType {
    CCM_RESP_PARAMS = 0x80,
    CCM_RESP_TELEM = 0x81,
} __attribute__((packed)) CcmResponseType;
assert_size(CcmResponseType, 1);

typedef struct CcmCurrentTelemetry {
    uint16_t bus_voltage_mv;
    uint16_t motor_voltage_cmd_mv;

    uint16_t current_setpoint_ma;
    uint8_t reserved[2];
    uint16_t current_samples_ma[20];
} CcmCurrentTelemetry;
assert_size(CcmCurrentTelemetry, 48);

typedef struct CcmVelocityTelemetry {
    float vel_setpoint_rads;
    float wheel_vel_rads;
} CcmVelocityTelemetry;
assert_size(CcmVelocityTelemetry, 8);

typedef struct CcmTelemetry {
    uint16_t master_error : 1;
    uint16_t hall_power_error : 1;
    uint16_t hall_disconnected_error : 1;
    uint16_t bldc_transition_error : 1;
    uint16_t bldc_commutation_watchdog_error : 1;
    uint16_t enc_disconnected_error: 1;
    uint16_t overcurrent_error : 1;
    uint16_t undervoltage_error : 1;
    uint16_t overvoltage_error : 1;
    uint16_t torque_limited : 1;
    uint16_t control_loop_time_error: 1;
    uint16_t reset_watchdog_independent: 1;
    uint16_t reset_watchdog_window: 1;
    uint16_t reset_low_power: 1;
    uint16_t reset_software: 1;
    uint16_t reset_pin: 1;

    CcmMotionControlType motion_control_type;
    uint8_t gain_stage_index : 8;

    CcmCurrentTelemetry current_telemetry;
    CcmVelocityTelemetry velocity_telemetry;

} CcmTelemetry;
assert_size(CcmTelemetry, 60);

typedef struct CcmResponse {
    CcmResponseType type;
    uint8_t seq_num;
    uint8_t _reserved[2];
    
    uint32_t timestamp;
    union CcmResponseData {
        CcmParameterPacket params;
        CcmTelemetry motion;
    } data;
} CcmResponse;
assert_size(CcmResponse, 68);