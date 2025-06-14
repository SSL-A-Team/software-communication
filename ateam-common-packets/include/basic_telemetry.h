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

#include "common.h"

typedef struct BasicTelemetry {
    uint16_t sequence_number;
    uint8_t robot_revision_major;
    uint8_t robot_revision_minor;

    uint32_t power_error : 1;
    uint32_t power_board_error : 1;
    uint32_t battery_error : 1;
    uint32_t battery_low : 1;
    uint32_t battery_crit : 1;
    uint32_t shutdown_pending : 1;
    uint32_t tipped_error : 1;
    uint32_t breakbeam_error : 1;
    uint32_t breakbeam_ball_detected : 1;
    uint32_t accelerometer_0_error : 1;
    uint32_t accelerometer_1_error: 1;
    uint32_t gyroscope_0_error : 1;
    uint32_t gyroscope_1_error : 1;
    uint32_t motor_fl_general_error : 1;
    uint32_t motor_fl_hall_error : 1;
    uint32_t motor_bl_general_error : 1;
    uint32_t motor_bl_hall_error : 1;
    uint32_t motor_br_general_error : 1;
    uint32_t motor_br_hall_error : 1;
    uint32_t motor_fr_general_error : 1;
    uint32_t motor_fr_hall_error : 1;
    uint32_t motor_drib_general_error : 1;
    uint32_t motor_drib_hall_error : 1;
    uint32_t kicker_board_error : 1;
    uint32_t chipper_available : 1;
    uint32_t kicker_available : 1;
    uint32_t body_vel_control_enabled : 1;
    uint32_t wheel_vel_control_enabled : 1;
    uint32_t wheel_torque_control_enabled : 1;
    uint32_t reserved : 3;
    
    uint16_t battery_percent;
    uint16_t kicker_charge_percent;
} BasicTelemetry;
assert_size(BasicTelemetry, 12);
