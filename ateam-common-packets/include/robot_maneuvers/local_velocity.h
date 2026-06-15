#pragma once

#include "../common.h"

typedef struct LocalVelocityCommand {
    float local_xd;
    float local_yd;
    float local_omega;
    float max_linear_acc;
    float max_angular_acc;
} LocalVelocityCommand;
assert_size(LocalVelocityCommand, 20);

typedef struct LocalVelocityTelemetry {
    uint32_t reserved;
} LocalVelocityTelemetry;
assert_size(LocalVelocityTelemetry, 4);

typedef struct ExtendedLocalVelocityTelemetry {
    LocalVelocityCommand cmd_echo;
} ExtendedLocalVelocityTelemetry;
assert_size(ExtendedLocalVelocityTelemetry, 20);