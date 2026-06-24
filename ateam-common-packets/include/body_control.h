#pragma once

#include "common.h"
#include "basic_control.h"

typedef union BodyControlManeuverExtendedTelemetry {
    ExtendedGlobalPositionTelemetry global_pos;
    ExtendedGlobalVelocityTelemetry global_vel;
    ExtendedLocalVelocityTelemetry local_vel;
    ExtendedGlobalAccelerationTelemetry global_acc;
    ExtendedLocalAccelerationTelemetry local_acc;
    ExtendedHeadingPivotTelemetry heading_pivot;
    ExtendedPointPivotTelemetry point_pivot;
} BodyControlManeuverExtendedTelemetry __attribute__((aligned (4)));

typedef struct BodyControlExtendedTelemetry {
    BodyControlMode body_control_mode;
    uint8_t vision_update: 1;
    uint8_t _reserved1: 7;
    uint8_t _reserved2[2];
    // 4 bytes

    BodyControlManeuverExtendedTelemetry maneuver; 

    float imu_gyro[3];  // rad/s
    float imu_accel[3];  // m/s^2
    float vision_pose[3];  // vision system pose measurement
    float body_traj_pos[3];  // computed trajectory position setpoint
    float body_traj_vel[3];  // computed trajectory velocity setpoint
    float kf_body_pos_prediction[3];  // Kalman Filter body position prediction
    float kf_body_vel_prediction[3];  // Kalman Filter body velocity prediction
    float kf_body_pos_estimate[3];  // Kalman Filter body position estimate
    float kf_body_vel_estimate[3];  // Kalman Filter body velocity estimate
    float body_vel_u[3];  // body velocity control outputs after control policy
    float body_accel_u[3];  // body accel control outputs after control policy
    float body_accel_u_fric_comp[3];  // body accel control outputs after control policy with friction compensation added in
} BodyControlExtendedTelemetry;
assert_size(BodyControlExtendedTelemetry, 4 + 28 + 12*3*4);