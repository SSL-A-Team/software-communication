// High-level helpers that used to wrap unsafe union access.  Now that the wire
// types use `#[repr(C, u8)]` discriminated unions, no unsafe is needed and most
// of the old wrappers are trivial one-liners or have been inlined at call sites.
//
// The old `ManeuverCommand`, `ManeuverTelemetry`, `ManeuverExtendedTelemetry`,
// `DataPacket`, and `TelemetryPacket` enums have been removed — callers match
// directly on `BodyControlCommand`, `BodyControlTelemetry`, etc.

use crate::packets::basic_control::BasicControl;
use crate::packets::basic_telemetry::BasicTelemetry;
use crate::packets::body_control::BodyControlExtendedTelemetry;
use crate::packets::robot_parameters::ParameterCommand;
use crate::packets::extended_telemetry::ExtendedTelemetry;
use crate::packets::error_telemetry::ErrorTelemetry;
use crate::is_basic_control_packet_safe;

#[derive(Copy, Clone)]
pub enum DataPacket {
    BasicControl(BasicControl),
    ParameterCommand(ParameterCommand),
}

#[derive(Copy, Clone)]
pub enum TelemetryPacket {
    Basic(BasicTelemetry),
    Extended(ExtendedTelemetry),
    ParameterCommandResponse(ParameterCommand),
    ErrorTelemetry(ErrorTelemetry),
}

pub fn is_data_packet_safe(pkt: &DataPacket) -> bool {
    match pkt {
        DataPacket::BasicControl(bc) => is_basic_control_packet_safe(bc),
        DataPacket::ParameterCommand(_) => true,
    }
}

impl BasicControl {
    /// Convenience accessor — equivalent to `&self.cmd` but matches the old API.
    pub fn get_maneuver_command(&self) -> &crate::packets::basic_control::BodyControlCommand {
        &self.cmd
    }
}

impl BasicTelemetry {
    pub fn set_maneuver_telemetry(&mut self, telem: crate::packets::basic_telemetry::BodyControlTelemetry) {
        self.control_telem = telem;
    }
}

impl BodyControlExtendedTelemetry {
    pub fn set_maneuver_telemetry(&mut self, telem: crate::packets::body_control::BodyControlManeuverExtendedTelemetry) {
        self.maneuver = telem;
    }
}
