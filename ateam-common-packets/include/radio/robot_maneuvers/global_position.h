#pragma once

#include "../../common.h"

typedef struct GlobalPositionCommand {
    float global_x;
    float global_y;
    float global_theta;
    float max_linear_vel;
    float max_angular_vel;
    float max_linear_acc;
    float max_angular_acc;
} GlobalPositionCommand;
assert_size(GlobalPositionCommand, 28);

typedef struct GlobalPositionTelemetry {
    uint32_t reserved;
} GlobalPositionTelemetry;
assert_size(GlobalPositionTelemetry, 4);

typedef struct ExtendedGlobalPositionTelemetry {
    GlobalPositionCommand cmd_echo;
} ExtendedGlobalPositionTelemetry;
assert_size(ExtendedGlobalPositionTelemetry, 28);