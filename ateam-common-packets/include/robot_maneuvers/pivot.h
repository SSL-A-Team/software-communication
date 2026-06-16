#pragma once

#include "../common.h"

typedef struct PivotCommand {
    float global_x;
    float global_y;
    float global_theta;
    float max_linear_vel;
    float max_angular_vel;
    float max_linear_acc;
    float max_angular_acc;
} PivotCommand;
assert_size(PivotCommand, 28);

typedef struct PivotTelemetry {
    uint32_t reserved;
} PivotTelemetry;
assert_size(PivotTelemetry, 4);

typedef struct ExtendedPivotTelemetry {
    PivotCommand cmd_echo;
} ExtendedPivotTelemetry;
assert_size(ExtendedPivotTelemetry, 28);
