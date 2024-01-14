use crate::bindings_radio::{BasicControl, ParameterCommand};

#[warn(dead_code)]
pub enum DataPacket {
    BasicControl(BasicControl),
    ParameterCommand(ParameterCommand),
}