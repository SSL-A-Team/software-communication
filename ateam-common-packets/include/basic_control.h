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
    uint32_t body_vel_controls_enabled : 1;
    uint32_t wheel_vel_control_enabled : 1;
    uint32_t wheel_torque_control_enabled : 1;
    uint32_t _reserved : 17;
    uint32_t play_song : 8;

    float vel_x_linear; // m/s
    float vel_y_linear; // m/s
    float vel_z_angular; // m/s
    float kick_vel; // m/s (also applies to chips)
    float dribbler_speed; // rpm
    KickRequest kick_request;
} BasicControl;
assert_size(BasicControl, 28);
