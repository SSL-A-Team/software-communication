use crate::bindings_radio::{BasicControl, ParameterCommand};

#[derive(Copy, Clone)]
pub enum DataPacket {
    BasicControl(BasicControl),
    ParameterCommand(ParameterCommand),
}

#[derive(Copy, Clone)]
pub enum TelemetryPacket {
    Basic(crate::bindings_radio::BasicTelemetry),
    Control(crate::bindings_radio::ControlDebugTelemetry),
    ParameterCommandResponse(crate::bindings_radio::ParameterCommand),
}