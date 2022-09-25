/**
 * @file radio.h
 * @author Austin Jones
 * @brief communication packet definitions for the radio firmware
 * @version 0.1
 * 
 * @copyright Copyright (c) 2022
 *
 */

#pragma once

#include <stdint.h>

// if C11 or newer is used, include assert_static macro for testing
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
#include <assert.h>
#endif

const uint16_t kProtocolVersionMajor = 0;
const uint16_t kProtocolVersionMinor = 0;

typedef enum CommandCode : unsigned char {
    CC_ACK = 1,
    CC_NACK = 2,
    CC_GOODBYE = 3,
    CC_HELLO = 101,
    CC_TELEMETRY = 102,
    CC_CONTROL = 201
} CommandCode_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(CommandCode_t) == 1, "Expected CommandCode_t to have a size of 1");
#endif 

typedef enum DataType : unsigned char {
    DT_NO_DATA = 0,
    DT_HELLO_DATA = 1,
    DT_BASIC_TELEMETRY = 2,
    DT_BASIC_CONTROL = 3
} DataType_t;  
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(DataType_t) == 1, "Expected DataType_t to have a size of 1");
#endif

/**
 * @brief A single packet to be sent over the network (via UDP)
 * 
 * @param major_version : uint16_t  : Major version number of protocol
 * @param minor_version : uint16_t  : Minor version number of protocol
 * @param command_code  : uint8_t  : Code describing data, see radio-protocol/README
 * @param data_type     : DataType : Type of message data
 * @param data_size     : uint16_t : Size of data in bytes
 * @param data          : uint8_t[500]    : Buffer containing data to send
 * 
 */
typedef struct RadioPacket {
    uint16_t major_version;
    uint16_t minor_version;
    CommandCode_t command_code;
    DataType_t data_type;
    uint16_t data_length;
    uint8_t data[500];
} RadioPacket_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(RadioPacket_t) <= 508, "Expected RadioPacket_t to have a size <= 508");
#endif
