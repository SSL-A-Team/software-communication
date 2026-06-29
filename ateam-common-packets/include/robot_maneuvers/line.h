#pragma once

#include "../common.h"

// Follow a line at a constant velocity while holding a fixed heading
// (BCM_HEADING_LINE).
//
// The line passes through (start_x, start_y) with direction (dir_x, dir_y)
// (normalized by the firmware). The robot drives perpendicular onto the line,
// rotates to global_theta, and accelerates along the line to line_velocity once
// within colinear_start_thresh of the line. Limit fields fall back to the
// firmware LinearParams defaults when left at zero.
typedef struct HeadingLineCommand {
    float start_x;                  // a point the line passes through (m)
    float start_y;
    float dir_x;                    // line direction (normalized by firmware)
    float dir_y;
    float line_velocity;            // target velocity along the line (m/s)
    float global_theta;             // target heading (rad)
    float max_vel_colinear;
    float max_vel_perp;
    float max_vel_angular;
    float max_accel_colinear;
    float max_accel_perp;
    float max_accel_angular;
    float colinear_start_thresh;    // perp distance to begin colinear accel (m)
} HeadingLineCommand;
assert_size(HeadingLineCommand, 52);

// Follow a line at a constant velocity while continuously facing a global point
// (BCM_POINT_LINE). Same line geometry as HeadingLineCommand, but instead of a
// fixed heading the robot continuously faces (target_x, target_y).
typedef struct PointLineCommand {
    float start_x;                  // a point the line passes through (m)
    float start_y;
    float dir_x;                    // line direction (normalized by firmware)
    float dir_y;
    float line_velocity;            // target velocity along the line (m/s)
    float target_x;                 // global point to face (m)
    float target_y;
    float max_vel_colinear;
    float max_vel_perp;
    float max_vel_angular;
    float max_accel_colinear;
    float max_accel_perp;
    float max_accel_angular;
    float colinear_start_thresh;    // perp distance to begin colinear accel (m)
} PointLineCommand;
assert_size(PointLineCommand, 56);

typedef struct HeadingLineTelemetry {
    uint32_t reserved;
} HeadingLineTelemetry;
assert_size(HeadingLineTelemetry, 4);

typedef struct PointLineTelemetry {
    uint32_t reserved;
} PointLineTelemetry;
assert_size(PointLineTelemetry, 4);

typedef struct ExtendedHeadingLineTelemetry {
    HeadingLineCommand cmd_echo;
} ExtendedHeadingLineTelemetry;
assert_size(ExtendedHeadingLineTelemetry, 52);

typedef struct ExtendedPointLineTelemetry {
    PointLineCommand cmd_echo;
} ExtendedPointLineTelemetry;
assert_size(ExtendedPointLineTelemetry, 56);
