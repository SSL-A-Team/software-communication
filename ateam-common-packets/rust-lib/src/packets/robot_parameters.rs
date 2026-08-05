#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParameterCommandCode {
    Read                = 0,
    Write               = 1,
    Ack                 = 2,
    NackInvalidName     = 3,
    NackInvalidTypeForName = 4,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParameterName {
    KfProcessStd        = 0,
    KfMeasurementStd    = 1,
    KfMaxState          = 2,
    PhysWheel           = 3,
    PhysInertia         = 4,
    PhysMotorModel      = 5,
    PhysFrictionModel   = 6,
    FrictionCompGating  = 7,
    PoseControlGain     = 8,
    TrajRecomputeError  = 9,
    PoseFbPidiiLinear   = 10,
    PoseFbPidiiAngular  = 11,
    TwistFbPidiiLinear  = 12,
    TwistFbPidiiAngular = 13,
}

/// Parameter data discriminated union. The variant discriminant encodes the
/// format (number of floats), replacing the old separate `data_format` field.
/// Layout: tag (1B) + padding (3B) + body (24B) = 28B.
#[repr(C, u8)]
#[derive(Clone, Copy, Debug)]
pub enum ParameterData {
    F32([f32; 1])       = 0,
    Vec2F32([f32; 2])   = 1,
    Vec3F32([f32; 3])   = 2,
    Vec4F32([f32; 4])   = 3,
    Vec5F32([f32; 5])   = 4,
    Vec6F32([f32; 6])   = 5,
}
const _: () = assert!(core::mem::size_of::<ParameterData>() == 28);

/// 28 → 32 bytes (`data_format` field removed; absorbed into `data` discriminant).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ParameterCommand {
    pub command_code:   ParameterCommandCode,
    pub parameter_name: ParameterName,
    pub _pad:           [u8; 2],
    pub data:           ParameterData,
}
const _: () = assert!(core::mem::size_of::<ParameterCommand>() == 32);
