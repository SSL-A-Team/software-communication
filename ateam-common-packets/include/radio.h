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

typedef enum DataType : unsigned char {
    ROBOT_MOTION_COMMAND,
    ROBOT_DEBUG
} DataType_t;  
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(DataType_t) == 1, "Expected DataType_t to have a size of 1");
#endif  

/**
 * @brief A single packet to be sent over the network (via UDP)
 * 
 * @param major_version : uint8_t  : Major version number of API
 * @param minor_version : uint8_t  : Minor version number of API
 * @param patch_version : uint16_t : Patch version number of API
 * @param command_code  : uint8_t  : Code describing data, see radio-protocol/README
 * @param data_type     : DataType : Type of message data
 * @param data_size     : uint16_t : Size of data in bytes
 * @param data          : void*    : Buffer containing data to send
 * 
 */
typedef struct RadioPacket {
    uint8_t major_version : 1;
    uint8_t minor_version : 1;
    uint16_t patch_version : 2;
    uint8_t command_code : 1;
    DataType data_type;
    uint16_t data_length : 2;
    void* data;
} RadioPacket_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(RadioPacket_t) <= 508, "Expected RadioPacket_t to have a size <= 508");
#endif