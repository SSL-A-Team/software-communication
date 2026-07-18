#pragma once

#include "../common.h"
#include "discovery.h"
#include "basic_control.h"
#include "basic_telemetry.h"
#include "error_telemetry.h"
#include "extended_telemetry.h"
#include "robot_parameters.h"
#include "../wire/power.h"
#include "../wire/stspin_current.h"

typedef enum CommandCode : uint8_t {
    // bidirectional commands and status
    CC_ACK = 1,
    CC_NACK = 2,
    CC_GOODBYE = 3,
    CC_KEEPALIVE = 4,

    // discovery
    CC_HELLO_REQ = 21,  // robot to multicast
    CC_HELLO_RESP = 22,  // software to robot direct

    // from robot to coach
    CC_TELEMETRY = 41,
    CC_CONTROL_DEBUG_TELEMETRY = 42,
    CC_ROBOT_PARAMETER_COMMAND = 43,
    CC_ERROR_TELEMETRY = 44,

    // from coach to robot
    CC_CONTROL = 61,
} CommandCode;
assert_size(CommandCode, 1);

typedef struct RadioHeader {
    uint32_t crc32;
    CommandCode command_code;
    uint8_t _reserved;
    uint16_t data_length;
} __attribute__((packed)) RadioHeader;
assert_size(RadioHeader, 8);

typedef union RadioData {
    HelloRequest hello_request;
    HelloResponse hello_response;
    BasicControl control;
    BasicTelemetry telemetry;
    ExtendedTelemetry extended_telemetry;
    ParameterCommand robot_parameter_command;
    ErrorTelemetry error_telemetry;
} RadioData;
assert_size(RadioData, 556);

// Same RadioPacket
typedef struct RadioPacket {
    RadioHeader header;
    RadioData data ;
} RadioPacket __attribute__((aligned (4)));
assert_size(RadioPacket, 564);
static_assert(sizeof(RadioPacket) <= 576);  // 576 is the current size limit of an entry in the packet buffer