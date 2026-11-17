#pragma once

#include "common.h"
#include "basic_control.h"

typedef union BodyControlManeuverExtendedTelemetry {
    ExtendedGlobalPositionTelemetry global_pos;
    ExtendedGlobalVelocityTelemetry global_vel;
    ExtendedLocalVelocityTelemetry local_vel;
    ExtendedGlobalAccelerationTelemetry global_acc;
    ExtendedLocalAccelerationTelemetry local_acc;
    ExtendedGlobalAccelerationTelemetry global_acc_open_loop;
    ExtendedLocalAccelerationTelemetry local_acc_open_loop;
    ExtendedHeadingPivotTelemetry heading_pivot;
    ExtendedPointPivotTelemetry point_pivot;
    ExtendedHeadingLineTelemetry heading_line;
    ExtendedPointLineTelemetry point_line;
} BodyControlManeuverExtendedTelemetry __attribute__((aligned (4)));

typedef struct BodyControlExtendedTelemetry {
    BodyControlMode body_control_mode;
    uint8_t vision_update: 1;  // a vision update was used during this control update
    uint8_t vision_old: 1;  // the vision measurement was determined older than the buffered EKF depth on robot receive
    uint8_t _reserved1: 6;
    uint8_t _reserved2[2];
    // 4 bytes

    uint32_t vision_insert_age_us;  // determined age of the vision measurement on robot receive
    // 4 bytes

    BodyControlManeuverExtendedTelemetry maneuver;  // active maneuver telemetry

    float imu_gyro_w;  // imu gyrometer heading angular velocity (rad/s)
    float imu_accel_xy[2];  // imu acccelerometer local frame x/y acceleration (m/s^2)
    float vision_pos[3];  // vision system position measurement, taken from the buffered EKF
    float pos_ekf[3];  // extended kalman filter (ekf) buffered position state
    float vel_ekf[3];  // extended kalman filter (ekf) buffered velocity state
    float pos_est[3];  // body position estimate, buffered ekf integrated to current time
    float vel_est[3];  // body velocity estimate, buffered ekf integrated to current time
    float traj_pos[3];  // trajectory position setpoint
    float traj_vel[3];  // trajectory velocity setpoint
    float traj_accel[3];  // trajectory acceleration setpoint
    float vel_u[3];  // body level velocity controller output
    float accel_u[3];  // body level acceleration controller output
} BodyControlExtendedTelemetry;
assert_size(BodyControlExtendedTelemetry, 4 + 4 + 56 + 3*4 + 10*3*4);