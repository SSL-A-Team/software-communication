#pragma once

#include "../common.h"

// Whether the robot drives forward or backward around the orbit.
typedef enum PivotDirection : uint8_t {
    // Velocity is acute with the heading (robot drives forward).
    PIVOT_DIRECTION_FORWARD = 0,
    // Velocity is obtuse with the heading (robot drives backward).
    PIVOT_DIRECTION_BACKWARD = 1,
} PivotDirection;
assert_size(PivotDirection, 1);

// Pivot toward a target heading (BCM_HEADING_PIVOT).
typedef struct HeadingPivotCommand {
    float global_theta;       // target heading (rad)
    float max_angular_vel;
    float max_angular_acc;
    float orbit_radius;
    // Angle (rad) between the orbit tangent and the heading, measured toward the
    // orbit center; absolute-valued. Ignored when compute_inset_angle != 0.
    float inset_angle;
    PivotDirection direction;
    // Bool: when nonzero, inset_angle is ignored and the inset is derived from a
    // linear model of the peak angular velocity (centrifugal lean).
    uint8_t compute_inset_angle;
    uint8_t reserved[2];
} HeadingPivotCommand;
assert_size(HeadingPivotCommand, 24);

// Pivot until facing a target point, accounting for translation (BCM_POINT_PIVOT).
typedef struct PointPivotCommand {
    float target_x;           // point to face after the pivot (m)
    float target_y;
    float max_angular_vel;
    float max_angular_acc;
    float orbit_radius;
    // Angle (rad) between the orbit tangent and the heading, measured toward the
    // orbit center; absolute-valued. Ignored when compute_inset_angle != 0.
    float inset_angle;
    PivotDirection direction;
    // Bool: when nonzero, inset_angle is ignored and the inset is derived from a
    // linear model of the peak angular velocity (centrifugal lean).
    uint8_t compute_inset_angle;
    uint8_t reserved[2];
} PointPivotCommand;
assert_size(PointPivotCommand, 28);

typedef struct HeadingPivotTelemetry {
    uint32_t reserved;
} HeadingPivotTelemetry;
assert_size(HeadingPivotTelemetry, 4);

typedef struct PointPivotTelemetry {
    uint32_t reserved;
} PointPivotTelemetry;
assert_size(PointPivotTelemetry, 4);

typedef struct ExtendedHeadingPivotTelemetry {
    HeadingPivotCommand cmd_echo;
} ExtendedHeadingPivotTelemetry;
assert_size(ExtendedHeadingPivotTelemetry, 24);

typedef struct ExtendedPointPivotTelemetry {
    PointPivotCommand cmd_echo;
} ExtendedPointPivotTelemetry;
assert_size(ExtendedPointPivotTelemetry, 28);
