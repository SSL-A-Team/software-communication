use crate::packets::basic_control::BasicControl;
use crate::packets::basic_telemetry::BasicTelemetry;
use crate::packets::discovery::{HelloRequest, HelloResponse};
use crate::packets::error_telemetry::ErrorTelemetry;
use crate::packets::extended_telemetry::ExtendedTelemetry;
use crate::packets::robot_parameters::ParameterCommand;

/// `command_code` removed from header (user decision); two reserved bytes keep
/// the struct at 8 bytes and the CRC/length fields at their original offsets.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RadioHeader {
    pub crc32:          u32,
    pub _reserved:      [u8; 2],
    pub data_length:    u16,
}
const _: () = assert!(core::mem::size_of::<RadioHeader>() == 8);

/// Radio packet data payload. Variant discriminant encodes the former
/// `CommandCode` tag byte. Layout: tag (1B) + padding (3B) + body (560B) = 564B.
#[repr(C, u8)]
#[derive(Clone, Copy, Debug)]
pub enum RadioData {
    HelloRequest(HelloRequest)          = 21,
    HelloResponse(HelloResponse)        = 22,
    Telemetry(BasicTelemetry)           = 41,
    ControlDebugTelemetry(ExtendedTelemetry) = 42,
    RobotParameterCommand(ParameterCommand) = 43,
    ErrorTelemetry(ErrorTelemetry)      = 44,
    Control(BasicControl)               = 61,
}
const _: () = assert!(core::mem::size_of::<RadioData>() == 564);

#[repr(C, align(4))]
#[derive(Clone, Copy, Debug)]
pub struct RadioPacket {
    pub header: RadioHeader,
    pub data:   RadioData,
}
const _: () = assert!(core::mem::size_of::<RadioPacket>() == 572);
// Hard limit: fits in a single 576-byte packet buffer entry.
const _: () = assert!(core::mem::size_of::<RadioPacket>() <= 576);
