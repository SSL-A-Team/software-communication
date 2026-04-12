/**
 * @file body_control.h
 * @author Nicholas Witten
 * @brief definition for body control data types
 * @version 0.1
 * 
 * @copyright Copyright (c) 2022
 *
 */
#pragma once

#include "common.h"


typedef enum BodyControlMode : uint8_t {
    BCM_OFF = 0,
    BCM_GLOBAL_POSE = 1,
    BCM_GLOBAL_TWIST = 2,
    BCM_LOCAL_TWIST = 3,
    BCM_GLOBAL_ACCEL = 4,
    BCM_LOCAL_ACCEL = 5
} BodyControlMode;
assert_size(BodyControlMode, 1);

typedef struct BodyControlTelemetry {
    BodyControlMode body_control_mode;
    uint8_t vision_update: 1;
    uint8_t _reserved1: 7;
    uint8_t _reserved2[2];
    // 4 bytes

    float imu_gyro[3];  // rad/s
    float imu_accel[3];  // m/s^2
    float vision_pose[3];  // vision system pose measurement
    float body_cmd[3];  // commanded body pose, twist, or accel
    float body_traj_pose[3];  // computed trajectory pose setpoint
    float body_traj_twist[3];  // computed trajectory twist setpoint
    float kf_body_pose_prediction[3];  // Kalman Filter body pose prediction
    float kf_body_twist_prediction[3];  // Kalman Filter body twist prediction
    float kf_body_pose_estimate[3];  // Kalman Filter body pose estimate
    float kf_body_twist_estimate[3];  // Kalman Filter body twist estimate
    float body_twist_u[3];  // body twist control outputs after control policy
    float body_accel_u[3];  // body accel control outputs after control policy
    float body_accel_u_fric_comp[3];  // body accel control outputs after control policy with friction compensation added in
} BodyControlTelemetry;
assert_size(BodyControlTelemetry, 4 + 13*3*4);