#pragma once

#include "../common.h"

typedef struct GlobalAccelerationCommand {
    float global_xdd;
    float global_ydd;
    float global_alpha;
} GlobalAccelerationCommand;
assert_size(GlobalAccelerationCommand, 12);

typedef struct GlobalAccelerationTelemetry {
    uint32_t reserved;
} GlobalAccelerationTelemetry;
assert_size(GlobalAccelerationTelemetry, 4);