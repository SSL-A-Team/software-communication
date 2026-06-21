#pragma once

#include "../common.h"

typedef struct PivotCommand {
    float global_theta;
    float max_angular_vel;
    float max_angular_acc;
    float orbit_radius;
    float inset_angle;
} PivotCommand;
assert_size(PivotCommand, 20);

typedef struct PivotTelemetry {
    uint32_t reserved;
} PivotTelemetry;
assert_size(PivotTelemetry, 4);

typedef struct ExtendedPivotTelemetry {
    PivotCommand cmd_echo;
} ExtendedPivotTelemetry;
assert_size(ExtendedPivotTelemetry, 20);
