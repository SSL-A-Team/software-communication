#pragma once

#include "common.h"
#include "body_control.h"
#include "kicker.h"
#include "power.h"
#include "stspin_current.h"

typedef struct ExtendedTelemetry {
    uint32_t timestamp_us_lo;
    uint32_t timestamp_us_hi;
    // 8 bytes

    PowerTelemetry power_status;
    // 28 bytes

    CcmTelemetry front_left_motor;
    CcmTelemetry back_left_motor;
    CcmTelemetry back_right_motor;
    CcmTelemetry front_right_motor;
    // 60 bytes each

    BodyControlExtendedTelemetry body_control_telemetry;
    // 160 bytes

    KickerTelemetry kicker_status;
    // 76 bytes
} ExtendedTelemetry;
assert_size(ExtendedTelemetry, 8 + 28 + 4*60 + 176 + 76);
