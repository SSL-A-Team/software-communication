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

#include "common.h"
#include "hello_data.h"
#include "basic_control.h"
#include "basic_telemetry.h"

const uint16_t kProtocolVersionMajor = 0;
const uint16_t kProtocolVersionMinor = 0;

typedef enum CommandCode : uint8_t {
    CC_ACK = 1,
    CC_NACK = 2,
    CC_GOODBYE = 3,
    CC_KEEPALIVE = 4,
    CC_HELLO_REQ = 101,
    CC_TELEMETRY = 102,
    CC_CONTROL = 201,
    CC_HELLO_RESP = 202,
} CommandCode;
assert_size(CommandCode, 1);

typedef struct RadioPacket {
    uint32_t crc32;
    uint16_t major_version;
    uint16_t minor_version;
    CommandCode command_code;
    uint16_t data_length;
    union Data {
        HelloRequest hello_request;
        HelloResponse hello_response;
        BasicControl control;
        BasicTelemetry telemetry;
    } data __attribute__((aligned (4)));
} RadioPacket;
assert_size(RadioPacket, 52);