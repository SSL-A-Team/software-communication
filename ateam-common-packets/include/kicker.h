#pragma once

#include "common.h"
#include "stspin.h"

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
    uint32_t telemetry_enabled: 1;
    uint32_t enable_charging: 1;
    uint32_t request_power_down: 1;
    uint32_t dribbler_mult: 8;
    // 21 bits reserved

    KickRequest kick_request;
    float kick_speed;
    float drib_speed;
} KickerControl;
assert_size(KickerControl, 16);

typedef struct KickerTelemetry {
    uint16_t error_detected : 1;
    uint16_t dribbler_error : 1;
    uint16_t power_down_requested : 1;
    uint16_t power_down_complete : 1;
    uint16_t ball_detected : 1;
    uint16_t charge_full : 1;
    uint16_t _reserved : 10;

    uint16_t charge_pct;
    float rail_voltage;
    float battery_voltage;

    unsigned char kicker_image_hash[4];

    MotorTelemetry dribbler_motor;
} KickerTelemetry;
assert_size(KickerTelemetry, 64);
