use crate::bindings::{BasicControl, ParameterCommand};

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
}