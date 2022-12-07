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

#include <stdint.h>

// if C11 or newer is used, include assert_static macro for testing
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L) && !defined(__ARM_ARCH_7EM__)
    #if defined(__ARM_ARCH_7EM__)
        #define static_assert(args...) _Static_assert(args...)
    #else
        #include <assert.h>
    #endif
#else
    #define static_assert(args...)
#endif

typedef float PidValue_t;

typedef enum MotorCommandPacketType {
    MCP_PARAMS = 0,
    MCP_MOTION = 1
} MotorCommandPacketType_t;

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
    PidValue_t vel_p;
    PidValue_t vel_i;
    PidValue_t vel_d;
    PidValue_t vel_i_max;
    PidValue_t cur_p;
    PidValue_t cur_i;
    PidValue_t cur_d;
    PidValue_t cur_i_max;
    uint16_t cur_clamp;
    // add future params here
} MotorCommand_Params_Packet_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(MotorCommand_Params_Packet_t) == 44, "Expected MotorCommand_Params_Packet_t to have a size of 44");
#endif

typedef enum MotorCommand_MotionType {
    OPEN_LOOP = 0,
    VELOCITY = 1,
    TORQUE = 2,
    BOTH= 3
} MotorCommand_MotionType_t;

typedef struct MotorCommand_Motion_Packet {
    uint32_t reset : 1;
    uint32_t enable_telemetry: 1;
    uint32_t reserved : 30;
    MotorCommand_MotionType_t motion_control_type;

    float setpoint;
} MotorCommand_Motion_Packet_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(MotorCommand_Motion_Packet_t) == 12, "Expected MotorCommand_Motion_Packet_t to have a size of 8");
#endif

typedef struct MotorCommandPacket {
    MotorCommandPacketType_t type;
    uint32_t crc32;
    union {
        MotorCommand_Params_Packet_t params;
        MotorCommand_Motion_Packet_t motion;
    };
} MotorCommandPacket_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(MotorCommandPacket_t) == 52, "Expected MotorCommandPacket_t to have a size of 52");
#endif

/////////////////
//  responses  //
/////////////////

typedef enum MotorResponsePacketType {
    MRP_PARAMS,
    MRP_MOTION,
    _MRP_FORCE_ENUM_TO_BE_4BYTES = 0xAA55CC33,
} MotorResponsePacketType_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(MotorResponsePacketType_t) == 4, "Expected MotorResponsePacketType_t to have a size of 4");
#endif

typedef struct MotorResponse_Params_Packet {
    uint8_t version_major;
    uint8_t version_minor;
    uint16_t version_patch;
    uint32_t timestamp;

    PidValue_t vel_p;
    PidValue_t vel_i;
    PidValue_t vel_d;
    PidValue_t vel_i_max;
    PidValue_t cur_p;
    PidValue_t cur_i;
    PidValue_t cur_d;
    PidValue_t torque_i_max;
    uint16_t cur_clamp;

    uint16_t git_dirty: 1;
    uint16_t reserved: 15;
} __attribute__((packed)) MotorResponse_Params_Packet_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(MotorResponse_Params_Packet_t) == 44, "Expected MotorResponse_Params_Packet to have a size of 44");
#endif

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
} __attribute__((packed)) MotorResponse_Motion_Packet_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(MotorResponse_Motion_Packet_t) == 48, "Expected MotorResponse_Motion_Packet to have a size of 48");
#endif

typedef struct MotorResponsePacket {
    MotorResponsePacketType_t type;
    uint32_t crc32;
    union {
        MotorResponse_Params_Packet_t params;
        MotorResponse_Motion_Packet_t motion;
    };
} __attribute__((packed)) MotorResponsePacket_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(MotorResponsePacket_t) == 56, "Expected MotorResponsePacket to have a size of 56");
#endif
