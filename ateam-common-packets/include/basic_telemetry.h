#pragma once

#include "common.h"
#include "body_control.h"

#include "robot_maneuvers/global_position.h"
#include "robot_maneuvers/global_velocity.h"
#include "robot_maneuvers/local_velocity.h"
#include "robot_maneuvers/global_acceleration.h"
#include "robot_maneuvers/local_acceleration.h"
#include "robot_maneuvers/pivot.h"

typedef union BodyControlTelemetry {
        GlobalPositionTelemetry global_pos;
        GlobalVelocityTelemetry global_vel;
        LocalVelocityTelemetry local_vel;
        GlobalAccelerationTelemetry global_acc;
        LocalAccelerationTelemetry local_acc;
        HeadingPivotTelemetry heading_pivot;
        PointPivotTelemetry point_pivot;
} BodyControlTelemetry __attribute__((aligned (4)));
assert_size(BodyControlTelemetry, 4);

typedef struct BasicTelemetry {
    uint8_t transmission_sequence_number;
    uint8_t control_data_sequence_number;
    BodyControlMode body_control_mode;
    uint8_t reserved1;
    // 4 bytes

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
    uint32_t controller_reset : 1;
    uint32_t _reserved : 5;
    // 4 bytes

    uint16_t battery_percent;
    uint16_t kicker_charge_percent;
    // 4 bytes

    // Body control mode telemetry
    BodyControlTelemetry control_telem;
    // 4 bytes

    // Quantized KF state estimate (mirrors kf_body_pos/vel_estimate in BodyControlExtendedTelemetry)
    int16_t kf_body_pos_estimate[3];  // [x (mm), y (mm), theta (mrad)], 1 LSB per unit
    int16_t kf_body_vel_estimate[3];  // [vx (mm/s), vy (mm/s), omega (mrad/s)], 1 LSB per unit
    // 12 bytes
} BasicTelemetry;
assert_size(BasicTelemetry, 4 + 4 + 4 + 4 + 6 + 6);
