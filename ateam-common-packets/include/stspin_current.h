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

typedef enum CurrentControlledMotor_CommandType {
    CCM_CMD_PARAMS = 0x20,
    CCM_CMD_MOTION = 0x21
} __attribute__((packed)) CurrentControlledMotor_CommandType;
assert_size(CurrentControlledMotor_CommandType, 1);

typedef enum CurrentControlledMotor_Parameter {
    CCM_PARAM_CURRENT_PI_CONTROLLER_GAIN_P = 0,
    CCM_PARAM_CURRENT_PI_CONTROLLER_GAIN_I = 1,
    CCM_PARAM_CURRENT_PI_CONTROLLER_ERR_ILIM = 2,
    CCM_PARAM_CURRENT_PI_CONTROLLER_ANTIJITTER = 3,
    CCM_PARAM_FIRMWARE_IMAGE_HASH = 100,
} __attribute__((packed)) CurrentControlledMotor_Parameter;
assert_size(CurrentControlledMotor_Parameter, 1);

typedef enum CurrentControlledMotor_ParameterOperation {
    CCM_PARAMOP_READ = 0,
    CCM_PARAMOP_WRITE = 1,
} __attribute__((packed)) CurrentControlledMotor_ParameterOperation;
assert_size(CurrentControlledMotor_ParameterOperation, 1);

typedef enum CurrentControlledMotor_ParameterDirection {
    CCM_PARAMDIR_COMMAND = 0,
    CCM_PARAMDIR_REPLY = 1,
} __attribute__((packed)) CurrentControlledMotor_ParameterDirection;
assert_size(CurrentControlledMotor_ParameterDirection, 1);

typedef union CurrentControlledMotor_ParameterValue {
    uint32_t val_u32;
    int32_t val_i32;
    float val_f32;
    uint8_t val_u8x4[4];
} CurrentControlledMotor_ParameterValue;

typedef struct CurrentControlledMotor_ParameterPacket {
    CurrentControlledMotor_Parameter parameter;
    CurrentControlledMotor_ParameterOperation parameter_operation;
    CurrentControlledMotor_ParameterDirection parameter_direction;
    uint8_t _reserved[1];
    CurrentControlledMotor_ParameterValue value;
} __attribute__((packed)) CurrentControlledMotor_ParameterPacket;
assert_size(CurrentControlledMotor_ParameterPacket, 8);

typedef enum CurrentControlledMotor_MotionControlType {
    CCM_MCT_MOTOR_OFF = 0,
    CCM_MCT_DUTY_OPENLOOP = 1,
    CCM_MCT_VOLTAGE_OPENLOOP = 2,
    CCM_MCT_CURRENT = 3,
    CCM_MCT_VELOCITY = 4,
    CCM_MCT_VELOCITY_CURRENT = 5
} __attribute__((packed)) CurrentControlledMotor_MotionControlType;
assert_size(CurrentControlledMotor_MotionControlType, 1);

typedef struct CurrentControlledMotor_MotionCommand {
    uint32_t reset : 1;
    uint32_t enable_telemetry: 1;
    uint32_t enable_motion: 1;
    uint32_t calibrate_current: 1;
    uint32_t _reserved : 28;

    CurrentControlledMotor_MotionControlType motion_control_type;
    uint8_t _reserved_2[1];
    int16_t current_setpoint_ma;

    float setpoint;
} __attribute__((__packed__)) CurrentControlledMotor_MotionCommand;
assert_size(CurrentControlledMotor_MotionCommand, 12); // Note: Same length as MotorCommand_Params_Packet

typedef struct CurrentControlledMotor_Command {
    CurrentControlledMotor_CommandType type;
    uint8_t _reserved[3];
    uint32_t crc32;
    union CurrentControlledMotor_CommandData {
        CurrentControlledMotor_ParameterPacket param;
        CurrentControlledMotor_MotionCommand motion;
    } data;
} __attribute__((__packed__)) CurrentControlledMotor_Command;
assert_size(CurrentControlledMotor_Command, 20);

/////////////////
//  responses  //
/////////////////

typedef enum CurrentControlledMotor_ResponseType {
    CCM_RESP_PARAMS = 0x80,
    CCM_RESP_TELEM = 0x81,
} __attribute__((packed)) CurrentControlledMotor_ResponseType;
assert_size(CurrentControlledMotor_ResponseType, 1);

typedef struct CurrentControlledMotor_CurrentTelemetry {
    uint16_t bus_voltage_mv;
    uint16_t motor_voltage_cmd_mv;

    uint16_t current_setpoint_ma;
    uint8_t reserved[2];
    uint16_t current_samples_ma[20];
} CurrentControlledMotor_CurrentTelemetry;
assert_size(CurrentControlledMotor_CurrentTelemetry, 48);

typedef struct CurrentControlledMotor_VelocityTelemetry {
    float vel_setpoint_rads;
    float wheel_vel_rads;
} CurrentControlledMotor_VelocityTelemetry;
assert_size(CurrentControlledMotor_VelocityTelemetry, 8);

typedef struct CurrentControlledMotor_Telemetry {
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

    CurrentControlledMotor_MotionControlType motion_control_type;
    uint8_t gain_stage_index : 8;

    CurrentControlledMotor_CurrentTelemetry current_telemetry;
    CurrentControlledMotor_VelocityTelemetry velocity_telemetry;

} CurrentControlledMotor_Telemetry;
assert_size(CurrentControlledMotor_Telemetry, 60);

typedef struct CurrentControlledMotor_Response {
    CurrentControlledMotor_ResponseType type;
    uint8_t seq_num;
    uint8_t _reserved[2];
    
    uint32_t timestamp;
    union CurrentControlledMotor_ResponseData {
        CurrentControlledMotor_ParameterPacket params;
        CurrentControlledMotor_Telemetry motion;
    } data;
} CurrentControlledMotor_Response;
assert_size(CurrentControlledMotor_Response, 68);