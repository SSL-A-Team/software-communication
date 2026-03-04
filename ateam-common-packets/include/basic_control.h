/**
 * @file basic_control.h
 * @author Matthew Barulic
 * @brief definition for Basic Control data type
 * @version 0.1
 * 
 * @copyright Copyright (c) 2022
 *
 */

#pragma once

#include "common.h"
#include "kicker.h"

typedef struct BasicControl {
    uint32_t request_shutdown : 1;
    uint32_t reboot_robot : 1;
    uint32_t game_state_in_stop : 1;
    uint32_t emergency_stop : 1;
    uint32_t body_pose_control_enabled : 1;
    uint32_t body_twist_control_enabled : 1;
    uint32_t body_accel_control_enabled : 1;
    uint32_t wheel_vel_control_enabled : 1;
    uint32_t wheel_torque_control_enabled : 1;
    uint32_t vision_update: 1;
    uint32_t dribbler_multiplier : 8;
    uint32_t _reserved : 6;
    uint32_t play_song : 8;

    float pose_x_linear_vision; // m
    float pose_y_linear_vision; // m
    float pose_z_angular_vision; // rad

    float x_linear_cmd; // m, m/s, m/s^2 (depending on control mode)
    float y_linear_cmd; // m, m/s, m/s^2 (depending on control mode)
    float z_angular_cmd; // rad, rad/s, rad/s^2 (depending on control mode)
    float kick_vel; // m/s (also applies to chips)
    float dribbler_speed; // rpm
    KickRequest kick_request;
} BasicControl;
assert_size(BasicControl, 40);
