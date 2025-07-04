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

typedef enum MotorCommandType {
    MCP_PARAMS = 0x20,
    MCP_MOTION = 0x21
} __attribute__((packed)) MotorCommandType;
assert_size(MotorCommandType, 1);

typedef struct ParameterMotorCommand {
    uint32_t update_timestamp : 1;
    uint32_t update_vel_p : 1;
    uint32_t update_vel_i : 1;
    uint32_t update_vel_d : 1;
    uint32_t update_vel_i_max: 1;
    uint32_t update_cur_p : 1;
    uint32_t update_cur_i : 1;
    uint32_t update_cur_d : 1;
    uint32_t update_cur_i_max: 1;
    uint32_t update_cur_clamp : 1;
    // add future flags here, decrement reserved
    uint32_t _reserved : 22;

    uint32_t timestamp;
    float vel_p;
    float vel_i;
    float vel_d;
    float vel_i_max;
    float cur_p;
    float cur_i;
    float cur_d;
    float cur_i_max;

    uint16_t cur_clamp;
    uint16_t _reserved_2;
    // add future params here
} ParameterMotorCommand;
assert_size(ParameterMotorCommand, 44);

typedef enum MotionCommandType {
    OPEN_LOOP = 0,
    VELOCITY = 1,
    TORQUE = 2,
    VELOCITY_W_TORQUE = 3
} __attribute__((packed)) MotionCommandType;
assert_size(MotionCommandType, 1);

typedef struct MotionMotorCommand {
    uint32_t reset : 1;
    uint32_t enable_telemetry: 1;
    uint32_t enable_motion: 1;
    uint32_t _reserved : 29;

    MotionCommandType motion_control_type;
    uint8_t _reserved_2[3];

    float setpoint;
    uint32_t _reserved_3[8];
} __attribute__((__packed__)) MotionMotorCommand;
assert_size(MotionMotorCommand, 44); // Note: Same length as MotorCommand_Params_Packet

typedef struct MotorCommandPacket {
    MotorCommandType type;
    uint8_t _reserved[3];
    uint32_t crc32;
    union CommandData {
        ParameterMotorCommand params;
        MotionMotorCommand motion;
    } data;
} MotorCommandPacket;
assert_size(MotorCommandPacket, 52);

/////////////////
//  responses  //
/////////////////

typedef enum MotorResponseType {
    MRP_PARAMS = 0x80,
    MRP_MOTION = 0x81,
} __attribute__((packed)) MotorResponseType;
assert_size(MotorResponseType, 1);

typedef struct ParameterMotorResponse {
    uint8_t version_major;
    uint8_t version_minor;
    uint16_t version_patch;

    float vel_p;
    float vel_i;
    float vel_d;
    float vel_i_max;
    float cur_p;
    float cur_i;
    float cur_d;
    float torque_i_max;

    uint16_t cur_clamp;
    uint16_t _reserved;

    unsigned char wheel_img_hash[4];
    uint32_t _reserved_2[3];
} __attribute__((packed)) ParameterMotorResponse;
assert_size(ParameterMotorResponse, 56); // Note: Same length as MotorResponse_Params_Packet

typedef struct MotorTelemetry {
    uint32_t master_error : 1;
    uint32_t hall_power_error : 1;
    uint32_t hall_disconnected_error : 1;
    uint32_t bldc_transition_error : 1;
    uint32_t bldc_commutation_watchdog_error : 1;
    uint32_t enc_disconnected_error: 1;
    uint32_t enc_decoding_error : 1;
    uint32_t hall_enc_vel_disagreement_error: 1;
    uint32_t overcurrent_error : 1;
    uint32_t undervoltage_error : 1;
    uint32_t overvoltage_error : 1;
    uint32_t torque_limited : 1;
    uint32_t control_loop_time_error: 1;
    uint32_t reset_watchdog_independent: 1;
    uint32_t reset_watchdog_window: 1;
    uint32_t reset_low_power: 1;
    uint32_t reset_software: 1;
    uint32_t reset_pin: 1;
    uint32_t _reserved : 6;
    uint32_t gain_stage_index : 8;

    float vel_setpoint;
    float vel_computed_rads;
    int32_t encoder_delta;
    float vel_enc_estimate;
    float vel_computed_error;
    float vel_computed_duty;

    float torque_setpoint;
    float current_estimate;
    float torque_estimate;
    float torque_computed_error;
    float torque_computed_nm;
    float torque_computed_duty;
    float vbus_voltage;
} __attribute__((packed)) MotorTelemetry;
assert_size(MotorTelemetry, 56);

typedef struct MotorResponse {
    MotorResponseType type;
    uint8_t _reserved[3];
    uint32_t timestamp;
    union ResponseData {
        ParameterMotorResponse params;
        MotorTelemetry motion;
    } data;
} __attribute__((packed)) MotorResponse;
assert_size(MotorResponse, 64);
