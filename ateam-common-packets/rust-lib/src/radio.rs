use crate::bindings::{BasicControl, MotorResponse_Motion_Packet, ParameterCommand};

#[derive(Copy, Clone)]
pub enum DataPacket {
    BasicControl(BasicControl),
    ParameterCommand(ParameterCommand),
}

#[derive(Copy, Clone)]
pub enum TelemetryPacket {
    Basic(crate::bindings::BasicTelemetry),
    Control(crate::bindings::ControlDebugTelemetry),
    ParameterCommandResponse(crate::bindings::ParameterCommand),
}

pub fn get_motor_response_motion_packet() -> MotorResponse_Motion_Packet {
    MotorResponse_Motion_Packet {
        _bitfield_align_1: [],
        _bitfield_1: MotorResponse_Motion_Packet::new_bitfield_1(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0),
        vel_setpoint: 0.0,
        vel_setpoint_clamped: 0.0,
        encoder_delta: 0,
        vel_enc_estimate: 0.0,
        vel_computed_error: 0.0,
        vel_computed_setpoint: 0.0,
        torque_setpoint: 0.0,
        current_estimate: 0.0,
        torque_estimate: 0.0,
        torque_computed_error: 0.0,
        torque_computed_setpoint: 0.0,
        vbus_voltage: 0.0,
    }
}