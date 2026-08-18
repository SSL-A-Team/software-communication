#pragma once

#include "common.h"

typedef enum CcmCommandType {
    CCM_CMD_PARAMS = 0x20,
    CCM_CMD_MOTION = 0x21
} __attribute__((packed)) CcmCommandType;
assert_size(CcmCommandType, 1);

typedef enum CcmParameter {
    CCM_PARAM_CURRENT_PI_CONTROLLER_GAIN_P = 0,
    CCM_PARAM_CURRENT_PI_CONTROLLER_GAIN_I = 1,
    CCM_PARAM_CURRENT_PI_CONTROLLER_ERR_ILIM = 2,
    CCM_PARAM_CURRENT_PI_CONTROLLER_ANTIJITTER = 3,
    CCM_PARAM_FIRMWARE_IMAGE_HASH = 100,
} __attribute__((packed)) CcmParameter;
assert_size(CcmParameter, 1);

typedef enum CcmParameterOperation {
    CCM_PARAMOP_READ = 0,
    CCM_PARAMOP_WRITE = 1,
} __attribute__((packed)) CcmParameterOperation;
assert_size(CcmParameterOperation, 1);

typedef enum CcmParameterDirection {
    CCM_PARAMDIR_COMMAND = 0,
    CCM_PARAMDIR_REPLY = 1,
} __attribute__((packed)) CcmParameterDirection;
assert_size(CcmParameterDirection, 1);

typedef union CcmParameterValue {
    uint32_t val_u32;
    int32_t val_i32;
    float val_f32;
    uint8_t val_u8x4[4];
} CcmParameterValue;

typedef struct CurrentControlledMotor_ParameterPacket {
    CcmParameter parameter;
    CcmParameterOperation parameter_operation;
    CcmParameterDirection parameter_direction;
    uint8_t _reserved[1];
    CcmParameterValue value;
} __attribute__((packed)) CcmParameterPacket;
assert_size(CcmParameterPacket, 8);

typedef enum CcmMotionControlType {
    CCM_MCT_MOTOR_OFF = 0,
    CCM_MCT_DUTY_OPENLOOP = 1,
    CCM_MCT_VOLTAGE_OPENLOOP = 2,
    CCM_MCT_CURRENT = 3,
    CCM_MCT_VELOCITY = 4,
    CCM_MCT_VELOCITY_CURRENT = 5
} __attribute__((packed)) CcmMotionControlType;
assert_size(CcmMotionControlType, 1);

typedef struct CcmMotionCommand {
    uint32_t reset : 1;
    uint32_t enable_telemetry: 1;
    uint32_t enable_motion: 1;
    uint32_t calibrate_current: 1;
    // selects the velocity source fed to the model-based current observer
    // (see CcmCurrentSenseTelemetry::current_model_ma). 0 = internal hall estimate,
    // 1 = external estimate (quadrature encoder). Targets without an encoder
    // ignore this and always use the hall estimate.
    uint32_t cs_vel_source_encoder : 1;
    uint32_t _reserved : 27;

    CcmMotionControlType motion_control_type;
    uint8_t _reserved_2[1];
    int16_t current_setpoint_ma;

    float setpoint;
} __attribute__((__packed__)) CcmMotionCommand;
assert_size(CcmMotionCommand, 12); // Note: Same length as MotorCommand_Params_Packet

typedef struct CcmCommand {
    CcmCommandType type;
    uint8_t _reserved[3];
    uint32_t crc32;
    union CcmCommandData {
        CcmParameterPacket param;
        CcmMotionCommand motion;
    } data;
} __attribute__((__packed__)) CcmCommand;
assert_size(CcmCommand, 20);

/////////////////
//  responses  //
/////////////////

typedef enum CcmResponseType {
    CCM_RESP_PARAMS = 0x80,
    CCM_RESP_TELEM = 0x81,
} __attribute__((packed)) CcmResponseType;
assert_size(CcmResponseType, 1);

typedef struct CcmCurrentTelemetry {
    uint16_t bus_voltage_mv;
    uint16_t motor_voltage_cmd_mv;

    int16_t current_setpoint_ma;
    int16_t hall_vel_est_drads;
    uint16_t current_samples_ma[20];
} CcmCurrentTelemetry;
assert_size(CcmCurrentTelemetry, 48);

////////////////////////////////////////
//  current sense estimator telemetry //
////////////////////////////////////////

// Bit flags for CcmCurrentSenseTelemetry::cs_flags. These describe how the
// firmware produced the concurrent current estimates so the control board can
// compare them without knowing which motor image is flashed.

// Phase 1 synchronous sampling is compiled in: the ADC trigger tracks duty and
// fires inside the PWM on-window, so current_unfilt_ma is real phase current and
// the control loop runs on it. When clear, current_unfilt_ma is sampled in
// freewheel and carries no signal, and the control loop runs on
// current_filt_ma.
#define CCM_CS_FLAG_SYNC_SAMPLING   (1U << 0)
// The duty correction denominator was above the usable floor this frame.
// When clear, current_duty_corrected_ma is unreliable (duty too small).
#define CCM_CS_FLAG_DUTY_CORR_VALID (1U << 1)
// The model observer used the external (encoder) velocity estimate.
// When clear it used the internal hall estimate.
#define CCM_CS_FLAG_VEL_SRC_EXT     (1U << 2)
// The selected velocity source reported a valid estimate this frame.
#define CCM_CS_FLAG_VEL_EST_VALID   (1U << 3)
// The model observer produced a usable value (motor output enabled).
#define CCM_CS_FLAG_MODEL_VALID     (1U << 4)

// Denominator for CcmCurrentSenseTelemetry::duty_arr. This is the motor controller's
// TIM1 ARR reload value, which is fixed by the 48 MHz sysclk and 40 kHz
// center-aligned PWM: 48e6 / 40e3 / 2 = 600.
#define CCM_CS_DUTY_ARR_FULL_SCALE 600U

// Current sense investigation instrumentation.
//
// This deliberately lives outside CcmTelemetry. CcmTelemetry is embedded five
// times in RadioPacket (four wheels plus the kicker's dribbler), which is within
// 12 bytes of the 576 byte radio packet buffer limit, so it cannot grow. These
// fields ride the motor UART only and stop at the control board, which is where
// the comparison is done.
typedef struct CcmCurrentSenseTelemetry {
    // Four concurrent estimates of the same physical quantity, all in mA.
    // See CURRENT_SENSING_INVESTIGATION.md.
    //
    //   current_filt_ma           - ADC CH4 / PA4, the amplifier output through
    //                               the external ~2 kHz RC low pass. Its time
    //                               average is bus current, D * I_phase, because
    //                               the shunt only conducts during the active
    //                               vector. This is the reading the control loop
    //                               has always used.
    //   current_unfilt_ma         - ADC CH3 / PA3, the amplifier output with no
    //                               external filter, sampled at the ADC trigger
    //                               instant. Real phase current only when
    //                               CCM_CS_FLAG_SYNC_SAMPLING is set; otherwise
    //                               the trigger sits in freewheel and this reads
    //                               noise around zero.
    //   current_duty_corrected_ma - Phase 0: current_filt_ma * ARR / D_arr.
    //                               An arithmetic estimate of phase current from
    //                               the bus current tap.
    //   current_model_ma          - Phase 3: (D * V_bus - Ke * w) / R_loop.
    //                               Signed, because plugging (rotor turning
    //                               against the commanded direction) yields a
    //                               negative estimate.
    uint16_t current_filt_ma;
    uint16_t current_unfilt_ma;
    uint16_t current_duty_corrected_ma;
    int16_t current_model_ma;

    // velocity actually fed to the model observer, deci-rad/s, motor shaft
    int16_t vel_est_used_drads;
    // mean effective on-time over the telemetry frame, in ARR counts.
    // duty = duty_arr / CCM_CS_DUTY_ARR_FULL_SCALE (600 at 40 kHz / 48 MHz).
    uint16_t duty_arr;

    uint8_t cs_flags;
    uint8_t _reserved;
    uint16_t _reserved_2;
} CcmCurrentSenseTelemetry;
assert_size(CcmCurrentSenseTelemetry, 16);

typedef struct CcmVelocityTelemetry {
    float vel_setpoint_rads;
    float wheel_vel_rads;
} CcmVelocityTelemetry;
assert_size(CcmVelocityTelemetry, 8);

typedef struct CcmTelemetry {
    uint16_t master_error : 1;
    uint16_t hall_power_error : 1;
    uint16_t hall_disconnected_error : 1;
    uint16_t bldc_transition_error : 1;
    uint16_t bldc_commutation_watchdog_error : 1;
    uint16_t enc_disconnected_error: 1;
    uint16_t overcurrent_error : 1;
    uint16_t undervoltage_error : 1;
    uint16_t overvoltage_error : 1;
    uint16_t torque_limited : 1;
    uint16_t control_loop_time_error: 1;
    uint16_t reset_watchdog_independent: 1;
    uint16_t reset_watchdog_window: 1;
    uint16_t reset_low_power: 1;
    uint16_t reset_software: 1;
    uint16_t reset_pin: 1;

    CcmMotionControlType motion_control_type;
    uint8_t gain_stage_index : 8;

    CcmCurrentTelemetry current_telemetry;
    CcmVelocityTelemetry velocity_telemetry;

} CcmTelemetry;
assert_size(CcmTelemetry, 60);

// What a motor actually sends back over its UART: the radio-shareable telemetry
// plus the control-board-local current sense instrumentation.
typedef struct CcmMotionTelemetry {
    CcmTelemetry telemetry;
    CcmCurrentSenseTelemetry current_sense;
} CcmMotionTelemetry;
assert_size(CcmMotionTelemetry, 76);

typedef struct CcmResponse {
    CcmResponseType type;
    uint8_t seq_num;
    uint8_t _reserved[2];
    
    uint32_t timestamp;
    union CcmResponseData {
        CcmParameterPacket params;
        CcmMotionTelemetry motion;
    } data;
} CcmResponse;
assert_size(CcmResponse, 84);