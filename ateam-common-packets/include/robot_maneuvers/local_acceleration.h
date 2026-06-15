#pragma once

#include "../common.h"

typedef struct LocalAccelerationCommand {
    float local_xdd;
    float local_ydd;
    float local_alpha;
} LocalAccelerationCommand;
assert_size(LocalAccelerationCommand, 12);

typedef struct LocalAccelerationTelemetry {
    uint32_t reserved;
} LocalAccelerationTelemetry;
assert_size(LocalAccelerationTelemetry, 4);

typedef struct ExtendedLocalAccelerationTelemetry {
    LocalAccelerationCommand cmd_echo;
} ExtendedLocalAccelerationTelemetry;
assert_size(ExtendedLocalAccelerationTelemetry, 12);