#pragma once

#include "../common.h"

typedef struct GlobalVelocityCommand {
    float global_xd;
    float global_yd;
    float global_omega;
    float max_linear_acc;
    float max_angular_acc;
} GlobalVelocityCommand;
assert_size(GlobalVelocityCommand, 20);

typedef struct GlobalVelocityTelemetry {
    uint32_t reserved;
} GlobalVelocityTelemetry;
assert_size(GlobalVelocityTelemetry, 4);