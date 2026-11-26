#pragma once

#include "common.h"

typedef enum ParameterCommandCode : uint8_t {
    PCC_READ = 0,
    PCC_WRITE = 1,
    PCC_ACK = 2,
    PCC_NACK_INVALID_NAME = 3,
    PCC_NACK_INVALID_TYPE_FOR_NAME = 4,
} ParameterCommandCode;
assert_size(ParameterCommandCode, 1);

typedef enum ParameterDataFormat : uint8_t {
    F32 = 0,
    VEC2_F32 = 1,
    VEC3_F32 = 2,
    VEC4_F32 = 3,
    VEC5_F32 = 4,
    VEC6_F32 = 5,
} ParameterDataFormat;
assert_size(ParameterDataFormat, 1);

typedef enum ParameterName : uint8_t {
    /// [POS_LINEAR, POS_ANGULAR, VEL_LINEAR]
    EKF_PROCESS_STD = 0,
    /// [VISION_STD_LINEAR, VISION_STD_ANGULAR]
    EKF_MEASUREMENT_STD = 1,
    /// [WHEEL_ANGLE_ALPHA, WHEEL_ANGLE_BETA, WHEEL_DISTANCE, WHEEL_RADIUS]
    /// Wheel alpha is the angle between robot +y axis and segment from robot origin to FL wheel
    /// Wheel beta is the angle between robot +y axis and segemnt from robot origin to BL wheel
    /// Wheel distance is the radial distance from robot origin to the wheels
    PHYS_WHEEL = 2,
    /// [BODY_MASS, BODY_MOMENT_Z]
    /// Body moment z is the angular moment of inertia around the robot's z axis
    PHYS_INERTIA = 3,
    /// [MOTOR_TORQUE_CONSTANT]
    PHYS_MOTOR_TORQUE_CONSTANT = 4,
    /// [COULOMB_LINEAR, COULOMB_ANGULAR, VISCOUS_LINEAR, VISCOUS_ANGULAR, EFFICIENCY_LINEAR, EFFICIENCY_ANGULAR]
    /// Linear terms are for sideways (local y) motion and scaled by a wheel-geometry direction factor otherwise
    PHYS_FRICTION_MODEL = 5,
    /// [K (1/s), MAX (m/s), ANTI_JITTER (m)]
    // XY position P feedback into the velocity setpoint
    FB_POS_LINEAR = 6,
    /// [POS_K (1/s), POS_MAX (rad/s), ANTI_JITTER (rad), VEL_K]
    // Angular position + velocity P feedback into the velocity setpoint
    FB_POS_ANGULAR = 7,
    /// [KP (mA/(rad/s)), KI (mA/rad), ANTI_JITTER (rad/s), MAX_CURRENT (A)]
    // Wheel velocity loop on the motor controllers. MAX_CURRENT caps the total (feed-forward + loop) wheel current
    FB_WHEEL_VEL = 8,
    /// [MAX_CURRENT (A)]
    // Feed-forward wheel current limit. The body accel output is scaled to stay under it
    FF_WHEEL_CURRENT_LIMIT = 9,
    /// [K ((rad/s)/A)]
    // Wheel slip feed-forward. Each wheel's velocity setpoint is raised by K * its feed-forward current, since the wheel surface outruns the ground under load
    FF_WHEEL_SLIP_COMP = 10,
    /// [ERROR_POS_LINEAR, ERROR_POS_ANGULAR, ERROR_VEL_LINEAR, ERROR_VEL_ANGULAR]
    ERROR_TRACKING_DIVERGENCE_THRESHOLD = 11,
    /// [SEED_SAMPLES]
    // Consistent vision samples needed to (re)seed the vision filter
    VISION_SEED_SAMPLES = 12,
} ParameterName;
assert_size(ParameterName, 1);

typedef struct ParameterCommand {
    ParameterCommandCode command_code;
    ParameterDataFormat data_format;
    ParameterName parameter_name;
    union ParameterData {
        float f32;
        float vec2_f32[2];
        float vec3_f32[3];
        float vec4_f32[4];
        float vec5_f32[5];
        float vec6_f32[6];
    } data __attribute__((aligned (4)));
} ParameterCommand;
assert_size(ParameterCommand, 28);