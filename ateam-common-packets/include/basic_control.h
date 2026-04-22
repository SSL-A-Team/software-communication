#pragma once

#include "common.h"
#include "kicker.h"

#include "robot_skills/global_position.h"
#include "robot_skills/global_velocity.h"
#include "robot_skills/local_velocity.h"
#include "robot_skills/global_acceleration.h"
#include "robot_skills/local_acceleration.h"

typedef enum BodyControlMode : uint8_t {
    BCM_OFF = 0,
    BCM_GLOBAL_POSITION = 1,
    BCM_GLOBAL_VELOCITY = 2,
    BCM_LOCAL_VELOCITY = 3,
    BCM_GLOBAL_ACCEL = 4,
    BCM_LOCAL_ACCEL = 5
    // add additional skills and modes here
} BodyControlMode;
assert_size(BodyControlMode, 1);

typedef union BodyControlCommand {
        GlobalPositionCommand global_pos;
        GlobalVelocityCommand global_vel;
        LocalVelocityCommand local_vel;
        GlobalAccelerationCommand global_acc;
        LocalAccelerationCommand local_acc;
} BodyControlCommand __attribute__((aligned (4)));
assert_size(BodyControlCommand, 28);

typedef struct BasicControl {
    // Bit field flags
    uint32_t request_shutdown : 1;
    uint32_t reboot_robot : 1;
    uint32_t game_state_in_stop : 1;
    uint32_t emergency_stop : 1;
    uint32_t wheel_vel_control_enabled : 1;
    uint32_t wheel_torque_control_enabled : 1;
    uint32_t vision_update: 1;
    uint32_t reserved1: 25;
    // 4 bytes

    // Vision update
    float vision_position_update[3];
    // 12 bytes

    // Control mode and ancillary data
    BodyControlMode body_control_mode;
    KickRequest kick_request;
    uint8_t play_song;
    uint8_t reserved2[1];
    // 4 bytes

    // Dribbler and kicker commands
    float kick_vel; // m/s (also applies to chips)
    float dribbler_speed; // rpm
    // 8 bytes

    // Body control command
    BodyControlCommand cmd;
    // 28 bytes
} BasicControl;
assert_size(BasicControl, 4 + 4 + 12 + 28 + 8);
