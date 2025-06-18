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
#include "stspin.h"


typedef struct ExtendedTelemetry {
    PowerTelemetry power_status;
    // 28 bytes

    MotorTelemetry front_left_motor;
    MotorTelemetry back_left_motor;
    MotorTelemetry back_right_motor;
    MotorTelemetry front_right_motor;
    // 48 bytes each

    KickerTelemetry kicker_status;
    // 60 bytes

    float imu_gyro[3];  // rad/s
    float imu_accel[3];  // m/s^2
    // 24 bytes

    float commanded_body_velocity[3];  // commanded body velocity from the AI
    float clamped_commanded_body_velocity[3];  // commanded body velocity from the AI after the local (firmware) velocity and acceleration limits are imposed
    float cgkf_body_velocity_state_estimate[3];  // CG Kalman Filter estiamted body velocity
    float body_velocity_u[3];  // body velocity PID output "u"
    /// 48 bytes

    float wheel_velocity_u[4];  // wheel velocities calculated from transform of body velocities after control policy
    float wheel_velocity_clamped_u[4];  // wheel velocities after control policy clamped for local acceleration limits
    /// 32 bytes
} ExtendedTelemetry;
assert_size(ExtendedTelemetry, 28 + 192 + 60 + 24 + 48 + 32);
