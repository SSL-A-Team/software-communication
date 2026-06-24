#pragma once

#include "../common.h"

// Whether the pivot is parameterized by a target heading or a target point to
// face after the orbit completes.
typedef enum PivotTargetMode : uint8_t {
    // Use global_theta as the final heading.
    PIVOT_TARGET_HEADING = 0,
    // Use (target_x, target_y) as a point to face after the pivot; the final
    // heading is solved to account for translation during the orbit.
    PIVOT_TARGET_POINT = 1,
} PivotTargetMode;
assert_size(PivotTargetMode, 1);

// Whether the robot drives forward or backward around the orbit.
typedef enum PivotDirection : uint8_t {
    // Velocity is acute with the heading (robot drives forward).
    PIVOT_DIRECTION_FORWARD = 0,
    // Velocity is obtuse with the heading (robot drives backward).
    PIVOT_DIRECTION_BACKWARD = 1,
} PivotDirection;
assert_size(PivotDirection, 1);

typedef struct PivotCommand {
    // Target heading (rad), used when target_mode == PIVOT_TARGET_HEADING.
    float global_theta;
    // Target point to face (m), used when target_mode == PIVOT_TARGET_POINT.
    float target_x;
    float target_y;
    float max_angular_vel;
    float max_angular_acc;
    float orbit_radius;
    // Angle (rad) between the orbit tangent and the heading, measured toward the
    // orbit center; absolute-valued. Ignored when compute_inset_angle != 0.
    float inset_angle;
    PivotTargetMode target_mode;
    PivotDirection direction;
    // Bool: when nonzero, inset_angle is ignored and the inset is derived from a
    // linear model of the peak angular velocity (centrifugal lean).
    uint8_t compute_inset_angle;
    uint8_t reserved;
} PivotCommand;
assert_size(PivotCommand, 32);

typedef struct PivotTelemetry {
    uint32_t reserved;
} PivotTelemetry;
assert_size(PivotTelemetry, 4);

typedef struct ExtendedPivotTelemetry {
    PivotCommand cmd_echo;
} ExtendedPivotTelemetry;
assert_size(ExtendedPivotTelemetry, 32);
