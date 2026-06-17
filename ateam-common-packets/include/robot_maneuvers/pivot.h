#pragma once

#include "../common.h"

typedef struct PivotCommand {
    float global_x_center;
    float global_y_center;
    float global_theta;
    float max_angular_vel;
    float max_angular_acc;
    float orbit_radius;
} PivotCommand;
assert_size(PivotCommand, 24);

typedef struct PivotTelemetry {
    uint32_t reserved;
} PivotTelemetry;
assert_size(PivotTelemetry, 4);

typedef struct ExtendedPivotTelemetry {
    PivotCommand cmd_echo;
} ExtendedPivotTelemetry;
assert_size(ExtendedPivotTelemetry, 24);
