#pragma once

#include "../common.h"
#include "stspin_current.h"

typedef enum KickRequest : uint8_t {
    KR_ARM,
    KR_DISABLE,
    KR_KICK_NOW,
    KR_KICK_TOUCH,
    KR_KICK_CAPTURED,
    KR_CHIP_NOW,
    KR_CHIP_TOUCH,
    KR_CHIP_CAPTURED
} KickRequest;

typedef enum DribblerCommand : uint8_t {
    DC_DISABLE      = 0,
    DC_HARD_RECEIVE = 1,
    DC_SOFT_RECEIVE = 2,
    DC_DRIBBLE      = 3,
    DC_VELOCITY     = 4,
    DC_CURRENT      = 5,
} DribblerCommand;

typedef struct KickerControl {
    uint16_t telemetry_enabled: 1;
    uint16_t enable_charging: 1;
    uint16_t request_power_down: 1;
    uint16_t bitfield_reserved1: 13;
    DribblerCommand dribbler_mode;

    KickRequest kick_request;
    float kick_speed;
    float drib_setpoint;
} KickerControl;
assert_size(KickerControl, 12);

typedef struct KickerTelemetry {
    uint16_t error_detected : 1;
    uint16_t dribbler_error : 1;
    uint16_t power_down_requested : 1;
    uint16_t power_down_complete : 1;
    uint16_t ball_detected : 1;
    uint16_t charge_full : 1;
    uint16_t dribbler_fw_loaded : 1;
    uint16_t _reserved : 9;

    uint16_t charge_pct;
    float rail_voltage;
    float battery_voltage;

    unsigned char kicker_image_hash[4];

    CcmTelemetry dribbler_motor;
} KickerTelemetry;
assert_size(KickerTelemetry, 76);
