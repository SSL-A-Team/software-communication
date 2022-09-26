/**
 * @file basic_telemetry.h
 * @author Matthew Barulic
 * @brief definition for Basic Telemetry data type
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

typedef struct BasicTelemetry {
    uint16_t sequence_number;
    uint8_t robot_revision_major;
    uint8_t robot_revision_minor;
    float battery_level; // volts
    float battery_temperature; // deg C
    uint32_t power_error : 1;
    uint32_t tipped_error : 1;
    uint32_t breakbeam_error : 1;
    uint32_t breakbeam_ball_detected : 1;
    uint32_t accelerometer_0_error : 1;
    uint32_t accelerometer_1_error: 1;
    uint32_t gyroscope_0_error : 1;
    uint32_t gyroscope_1_error : 1;
    uint32_t motor_0_general_error : 1;
    uint32_t motor_0_hall_error : 1;
    uint32_t motor_1_general_error : 1;
    uint32_t motor_1_hall_error : 1;
    uint32_t motor_2_general_error : 1;
    uint32_t motor_2_hall_error : 1;
    uint32_t motor_3_general_error : 1;
    uint32_t motor_3_hall_error : 1;
    uint32_t motor_4_general_error : 1;
    uint32_t motor_4_hall_error : 1;
    uint32_t chipper_available : 1;
    uint32_t kicker_available : 1;
    uint32_t reserved : 12;
    float motor_0_temperature; // deg C
    float motor_1_temperature; // deg C
    float motor_2_temperature; // deg C
    float motor_3_temperature; // deg C
    float motor_4_temperature; // deg C
    float kicker_charge_level; // volts
} BasicTelemetry_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(BasicTelemetry_t) == 40, "Expected BasicTelemetry_t to have a size of 40");
#endif
