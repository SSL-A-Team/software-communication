use crate::bitfields::BasicControlFlags;
use crate::packets::kicker::{DribblerCommand, KickRequest};
use crate::packets::maneuvers::*;

/// Body control mode discriminated union. The variant discriminant IS the wire
/// tag byte — no separate `body_control_mode` field needed in `BasicControl`.
///
/// Layout (from `#[repr(C, u8)]`): tag (1B) + padding (3B) + body (56B) = 60B.
#[repr(C, u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum BodyControlCommand {
    #[default]
    Off         = 0,
    EstopBrake  = 1,
    GlobalPosition(GlobalPositionCommand)       = 10,
    GlobalVelocity(GlobalVelocityCommand)       = 11,
    LocalVelocity(LocalVelocityCommand)         = 12,
    GlobalAcceleration(GlobalAccelerationCommand) = 13,
    LocalAcceleration(LocalAccelerationCommand) = 14,
    HeadingPivot(HeadingPivotCommand)           = 20,
    PointPivot(PointPivotCommand)               = 21,
    HeadingLine(HeadingLineCommand)             = 30,
    PointLine(PointLineCommand)                 = 31,
}
const _: () = assert!(core::mem::size_of::<BodyControlCommand>() == 60);

/// 84 → 88 bytes. Removed `body_control_mode` field; it is now the discriminant
/// embedded in `cmd` (at `cmd`'s offset 0 within the struct).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BasicControl {
    /// 9 control flags (request_shutdown … reset_controller).
    pub flags:                      BasicControlFlags,
    /// Vision measurement to fuse before executing this command.
    pub vision_position_update:     [f32; 3],
    pub kick_request:               KickRequest,
    pub play_song:                  u8,
    pub dribbler_mode:              DribblerCommand,
    pub _pad:                       u8,
    pub kick_vel:                   f32,
    pub dribbler_setpoint:          f32,
    /// Self-discriminating body control command; match on variants directly.
    pub cmd:                        BodyControlCommand,
}
const _: () = assert!(core::mem::size_of::<BasicControl>() == 88);
