/**
 * @file extended_telemetry.h
 * @author Will Stuckey
 * @brief definition for controls debug telemetry data type
 * @version 0.1
 *
 * @copyright Copyright (c) 2023
 *
 */

#pragma once

#include "common.h"
#include "kicker.h"
#include "power.h"
#include "stspin_current.h"

typedef struct ExtendedTelemetry {
    uint32_t timestamp_us_lo;
    uint32_t timestamp_us_hi;
    // 8 bytes

    uint32_t body_pose_control_enabled : 1;
    uint32_t body_twist_control_enabled : 1;
    uint32_t body_accel_control_enabled : 1;
    uint32_t vision_update : 1;
    uint32_t reserved : 28;
    // 4 bytes

    // PowerTelemetry power_status;
    // 28 bytes

    CcmTelemetry front_left_motor;
    CcmTelemetry back_left_motor;
    CcmTelemetry back_right_motor;
    CcmTelemetry front_right_motor;
    // 60 bytes each

    // KickerTelemetry kicker_status;
    // 64 bytes

    float imu_gyro[3];  // rad/s
    float imu_accel[3];  // m/s^2
    // 24 bytes

    float vision_pose[3];  // vision system pose measurement
    // 12 bytes

    float body_cmd[3];  // commanded body pose, twist, or accel
    float kf_body_pose_prediction[3];  // Kalman Filter body pose estimate
    float kf_body_twist_prediction[3];  // Kalman Filter body twist estimate
    float kf_body_pose_estimate[3];  // Kalman Filter body pose estimate
    float kf_body_twist_estimate[3];  // Kalman Filter body twist estimate
    float body_twist_u[3];  // body twist control outputs after control policy
    float body_accel_u[3];  // body accel control outputs after control policy
    /// 84 bytes
} ExtendedTelemetry;
// assert_size(ExtendedTelemetry, 8 + 4 + 28 + 4*60 + 64 + 24 + 12 + 84);
assert_size(ExtendedTelemetry, 8 + 4 + 4*60 + 24 + 12 + 84);
