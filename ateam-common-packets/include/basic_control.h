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
    float vel_x_linear; // m/s
    float vel_y_linear; // m/s
    float vel_z_angular; // m/s
    float kick_vel; // m/s (also applies to chips)
    float dribbler_speed; // rpm
    KickRequest kick_request;
} BasicControl;
assert_size(BasicControl, 24);
