#pragma once

#include "../common.h"

typedef struct GlobalAccelerationCommand {
    float local_xdd;
    float local_ydd;
    float local_alpha;
} GlobalAccelerationCommand;
assert_size(GlobalAccelerationCommand, 12);

typedef struct GlobalAccelerationTelemetry {
    uint32_t reserved;
} GlobalAccelerationTelemetry;
assert_size(GlobalAccelerationTelemetry, 4);