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

typedef enum MotorCommandPacketType {
    MCP_PARAMS = 0,
    MCP_MOTION = 1
} __attribute__((packed)) MotorCommandPacketType;
assert_size(MotorCommandPacketType, 1);

typedef struct MotorCommand_Params_Packet {
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
    uint32_t reserved : 22;

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
    // add future params here
} MotorCommand_Params_Packet;
assert_size(MotorCommand_Params_Packet, 44);

typedef enum MotorCommand_MotionType {
    OPEN_LOOP = 0,
    VELOCITY = 1,
    TORQUE = 2,
    BOTH= 3
} MotorCommand_MotionType;

typedef struct MotorCommand_Motion_Packet {
    uint32_t reset : 1;
    uint32_t enable_telemetry: 1;
    uint32_t reserved : 30;
    MotorCommand_MotionType motion_control_type;
    float setpoint;
} MotorCommand_Motion_Packet;
assert_size(MotorCommand_Motion_Packet, 12);

typedef struct MotorCommandPacket {
    MotorCommandPacketType type;
    uint32_t crc32;
    union CommandData {
        MotorCommand_Params_Packet params;
        MotorCommand_Motion_Packet motion;
    } data __attribute__((aligned (4)));
} MotorCommandPacket;
assert_size(MotorCommandPacket, 52);

/////////////////
//  responses  //
/////////////////

typedef enum MotorResponsePacketType {
    MRP_PARAMS,
    MRP_MOTION,
} __attribute__((packed)) MotorResponsePacketType;
assert_size(MotorResponsePacketType, 1);

typedef struct MotorResponse_Params_Packet {
    uint8_t version_major;
    uint8_t version_minor;
    uint16_t version_patch;
    uint32_t timestamp;

    float vel_p;
    float vel_i;
    float vel_d;
    float vel_i_max;
    float cur_p;
    float cur_i;
    float cur_d;
    float torque_i_max;
    uint16_t cur_clamp;

    uint16_t git_dirty: 1;
    uint16_t reserved: 15;
} __attribute__((packed)) MotorResponse_Params_Packet;
assert_size(MotorResponse_Params_Packet, 44);

typedef struct MotorResponse_Motion_Packet {
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
    uint32_t reserved : 19;

    uint32_t timestamp;

    float vel_setpoint;
    int32_t encoder_delta;
    float vel_enc_estimate;
    float vel_hall_estimate;
    float vel_computed_error;
    float vel_computed_setpoint;

    float current_setpoint;
    float current_estimate;
    float current_computed_error;
    float current_computed_setpoint;
} __attribute__((packed)) MotorResponse_Motion_Packet;
assert_size(MotorResponse_Motion_Packet, 48);

typedef struct MotorResponsePacket {
    MotorResponsePacketType type;
    uint32_t crc32;
    union ResponseData {
        MotorResponse_Params_Packet params;
        MotorResponse_Motion_Packet motion;
    } data __attribute__((aligned (4)));
} __attribute__((packed)) MotorResponsePacket;
assert_size(MotorResponsePacket, 56);
