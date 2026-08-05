use ateam_common_packets_macros::wire_bitfield;

// ─── BasicControl ────────────────────────────────────────────────────────────

#[wire_bitfield(u32)]
pub struct BasicControlFlags {
    pub request_shutdown:           bool,
    pub reboot_robot:               bool,
    pub game_state_in_stop:         bool,
    pub game_state_in_halt:         bool,
    pub emergency_stop:             bool,
    pub wheel_vel_control_enabled:  bool,
    pub wheel_torque_control_enabled: bool,
    pub vision_update:              bool,
    pub reset_controller:           bool,
    #[bits(23)]
    _reserved: u32,
}

// ─── BasicTelemetry ──────────────────────────────────────────────────────────

#[wire_bitfield(u32)]
pub struct BasicTelemetryErrors {
    pub power_error:                bool,
    pub power_board_error:          bool,
    pub battery_error:              bool,
    pub battery_low:                bool,
    pub battery_crit:               bool,
    pub shutdown_pending:           bool,
    pub tipped_error:               bool,
    pub breakbeam_error:            bool,
    pub breakbeam_ball_detected:    bool,
    pub accelerometer_0_error:      bool,
    pub accelerometer_1_error:      bool,
    pub gyroscope_0_error:          bool,
    pub gyroscope_1_error:          bool,
    pub motor_fl_general_error:     bool,
    pub motor_fl_hall_error:        bool,
    pub motor_bl_general_error:     bool,
    pub motor_bl_hall_error:        bool,
    pub motor_br_general_error:     bool,
    pub motor_br_hall_error:        bool,
    pub motor_fr_general_error:     bool,
    pub motor_fr_hall_error:        bool,
    pub motor_drib_general_error:   bool,
    pub motor_drib_hall_error:      bool,
    pub kicker_board_error:         bool,
    pub chipper_available:          bool,
    pub kicker_available:           bool,
    pub controller_reset:           bool,
    #[bits(5)]
    _reserved: u32,
}

// ─── BodyControlExtendedTelemetry ────────────────────────────────────────────

#[wire_bitfield(u8)]
pub struct BodyControlExtTelemetryFlags {
    pub vision_update: bool,
    #[bits(7)]
    _reserved: u8,
}

// ─── BatteryInfo ─────────────────────────────────────────────────────────────

#[wire_bitfield(u16)]
pub struct BatteryStatus {
    pub battery_ok:                 bool,
    pub battery_balance_connected:  bool,
    pub battery_low:                bool,
    pub battery_critical:           bool,
    pub battery_cell_low:           bool,
    pub battery_cell_critical:      bool,
    pub battery_cell_imbalance_warn: bool,
    #[bits(9)]
    _reserved: u16,
}

// ─── PowerTelemetry ──────────────────────────────────────────────────────────

#[wire_bitfield(u32)]
pub struct PowerStatus {
    pub power_ok:                       bool,
    pub power_rail_3v3_ok:              bool,
    pub power_rail_5v0_ok:              bool,
    pub power_rail_12v0_ok:             bool,
    pub high_current_operations_allowed: bool,
    pub shutdown_requested:             bool,
    #[bits(26)]
    _reserved: u32,
}

// ─── PowerCommand ────────────────────────────────────────────────────────────

#[wire_bitfield(u32)]
pub struct PowerCommandFlags {
    pub request_shutdown:   bool,
    pub cancel_shutdown:    bool,
    pub ready_shutdown:     bool,
    pub force_shutdown:     bool,
    #[bits(28)]
    _reserved: u32,
}

// ─── HelloRequest ────────────────────────────────────────────────────────────

#[wire_bitfield(u8)]
pub struct DiscoveryFlags {
    pub coms_repo_dirty:        bool,
    pub controls_repo_dirty:    bool,
    pub firmware_repo_dirty:    bool,
    #[bits(5)]
    _reserved: u8,
}

// ─── KickerControl ───────────────────────────────────────────────────────────

#[wire_bitfield(u16)]
pub struct KickerCommandFlags {
    pub telemetry_enabled:      bool,
    pub enable_charging:        bool,
    pub request_power_down:     bool,
    #[bits(13)]
    _reserved: u16,
}

// ─── KickerTelemetry ─────────────────────────────────────────────────────────

#[wire_bitfield(u16)]
pub struct KickerStatusFlags {
    pub error_detected:         bool,
    pub dribbler_error:         bool,
    pub power_down_requested:   bool,
    pub power_down_complete:    bool,
    pub ball_detected:          bool,
    pub charge_full:            bool,
    pub dribbler_fw_loaded:     bool,
    #[bits(9)]
    _reserved: u16,
}

// ─── CcmMotionCommand ────────────────────────────────────────────────────────

#[wire_bitfield(u32)]
pub struct CcmMotionCommandFlags {
    pub reset:                  bool,
    pub enable_telemetry:       bool,
    pub enable_motion:          bool,
    pub calibrate_current:      bool,
    #[bits(28)]
    _reserved: u32,
}

// ─── CcmTelemetry ────────────────────────────────────────────────────────────

#[wire_bitfield(u16)]
pub struct CcmTelemetryStatus {
    pub master_error:                       bool,
    pub hall_power_error:                   bool,
    pub hall_disconnected_error:            bool,
    pub bldc_transition_error:              bool,
    pub bldc_commutation_watchdog_error:    bool,
    pub enc_disconnected_error:             bool,
    pub overcurrent_error:                  bool,
    pub undervoltage_error:                 bool,
    pub overvoltage_error:                  bool,
    pub torque_limited:                     bool,
    pub control_loop_time_error:            bool,
    pub reset_watchdog_independent:         bool,
    pub reset_watchdog_window:              bool,
    pub reset_low_power:                    bool,
    pub reset_software:                     bool,
    pub reset_pin:                          bool,
}
