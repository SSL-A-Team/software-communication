use crate::{bindings::{BasicControl, ParameterCommand}, is_basic_control_packet_safe};

#[derive(Copy, Clone)]
pub enum DataPacket {
    BasicControl(BasicControl),
    ParameterCommand(ParameterCommand),
}

#[derive(Copy, Clone)]
pub enum TelemetryPacket {
    Basic(crate::bindings::BasicTelemetry),
    Extended(crate::bindings::ExtendedTelemetry),
    ParameterCommandResponse(crate::bindings::ParameterCommand),
    ErrorTelemetry(crate::bindings::ErrorTelemetry),
}

#[derive(Copy, Clone)]
pub enum ParameterType {
    Basic(crate::bindings::BasicTelemetry),
    Extended(crate::bindings::ExtendedTelemetry),
    ParameterCommandResponse(crate::bindings::ParameterCommand),
    ErrorTelemetry(crate::bindings::ErrorTelemetry),
}

pub fn is_data_packet_safe(data_packet: &DataPacket) -> bool {
    match data_packet {
        DataPacket::BasicControl(basic_control) => is_basic_control_packet_safe(basic_control),
        DataPacket::ParameterCommand(_parameter_command) => true,
    }
}