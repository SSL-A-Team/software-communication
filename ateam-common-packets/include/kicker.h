#pragma once

#include "common.h"

typedef enum KickRequest {
    KR_ARM,
    KR_DISABLE,
    KR_KICK_NOW,
    KR_KICK_TOUCH,
    KR_KICK_CAPTURED,
    KR_CHIP_NOW,
    KR_CHIP_TOUCH,
    KR_CHIP_CAPTURED
} KickRequest;

typedef struct KickerControl {
    uint32_t enable_charging: 1;
    uint32_t flag_kick: 1;
    uint32_t flag_chip: 1;
    uint32_t flag_power_down: 1;
    // 28 bits reserved
    
    float kick_speed;
} KickerControl;
assert_size(KickerControl, 8);

typedef struct KickerTelemetry {
    uint32_t powered_down: 1;
    uint32_t ball_detected: 1;
    uint32_t error_detected: 1;
    // 29 bits reserved

    float rail_voltage;
    float battery_voltage;
} KickerTelemetry;
assert_size(KickerTelemetry, 12);