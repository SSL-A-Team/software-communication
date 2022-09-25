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

#include <stdint.h>

// if C11 or newer is used, include assert_static macro for testing
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
#include <assert.h>
#endif

typedef enum KickRequest {
    KR_DISABLE,
    KR_KICK_NOW,
    KR_KICK_TOUCH,
    KR_KICK_CAPTURED,
    KR_CHIP_NOW,
    KR_CHIP_TOUCH,
    KR_CHIP_CAPTURED
} KickRequest_t;

typedef struct BasicControl {
    float vel_x_linear; // m/s
    float vel_y_linear; // m/s
    float vel_z_angular; // m/s
    float kick_vel; // m/s (also applies to chips)
    float dribbler_speed; // rpm
    uint8_t kick_request;
} BasicControl_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(BasicControl_t) == 24, "Expected BasicControl_t to have a size of 24");
#endif
