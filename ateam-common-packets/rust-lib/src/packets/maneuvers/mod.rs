use nalgebra::Vector3;

// ─── Global Position ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GlobalPositionCommand {
    pub global_x:           f32,
    pub global_y:           f32,
    pub global_theta:       f32,
    pub max_linear_vel:     f32,
    pub max_angular_vel:    f32,
    pub max_linear_acc:     f32,
    pub max_angular_acc:    f32,
}
const _: () = assert!(core::mem::size_of::<GlobalPositionCommand>() == 28);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GlobalPositionTelemetry {
    pub reserved: u32,
}
const _: () = assert!(core::mem::size_of::<GlobalPositionTelemetry>() == 4);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExtendedGlobalPositionTelemetry {
    pub cmd_echo: GlobalPositionCommand,
}
const _: () = assert!(core::mem::size_of::<ExtendedGlobalPositionTelemetry>() == 28);

// ─── Global Velocity ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GlobalVelocityCommand {
    pub global_xd:          f32,
    pub global_yd:          f32,
    pub global_omega:       f32,
    pub max_linear_acc:     f32,
    pub max_angular_acc:    f32,
}
const _: () = assert!(core::mem::size_of::<GlobalVelocityCommand>() == 20);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GlobalVelocityTelemetry {
    pub reserved: u32,
}
const _: () = assert!(core::mem::size_of::<GlobalVelocityTelemetry>() == 4);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExtendedGlobalVelocityTelemetry {
    pub cmd_echo: GlobalVelocityCommand,
}
const _: () = assert!(core::mem::size_of::<ExtendedGlobalVelocityTelemetry>() == 20);

// ─── Local Velocity ──────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LocalVelocityCommand {
    pub local_xd:           f32,
    pub local_yd:           f32,
    pub local_omega:        f32,
    pub max_linear_acc:     f32,
    pub max_angular_acc:    f32,
}
const _: () = assert!(core::mem::size_of::<LocalVelocityCommand>() == 20);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalVelocityTelemetry {
    pub reserved: u32,
}
const _: () = assert!(core::mem::size_of::<LocalVelocityTelemetry>() == 4);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExtendedLocalVelocityTelemetry {
    pub cmd_echo: LocalVelocityCommand,
}
const _: () = assert!(core::mem::size_of::<ExtendedLocalVelocityTelemetry>() == 20);

// ─── Global Acceleration ─────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GlobalAccelerationCommand {
    pub global_xdd:     f32,
    pub global_ydd:     f32,
    pub global_alpha:   f32,
}
const _: () = assert!(core::mem::size_of::<GlobalAccelerationCommand>() == 12);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GlobalAccelerationTelemetry {
    pub reserved: u32,
}
const _: () = assert!(core::mem::size_of::<GlobalAccelerationTelemetry>() == 4);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExtendedGlobalAccelerationTelemetry {
    pub cmd_echo: GlobalAccelerationCommand,
}
const _: () = assert!(core::mem::size_of::<ExtendedGlobalAccelerationTelemetry>() == 12);

// ─── Local Acceleration ──────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LocalAccelerationCommand {
    pub local_xdd:      f32,
    pub local_ydd:      f32,
    pub local_alpha:    f32,
}
const _: () = assert!(core::mem::size_of::<LocalAccelerationCommand>() == 12);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalAccelerationTelemetry {
    pub reserved: u32,
}
const _: () = assert!(core::mem::size_of::<LocalAccelerationTelemetry>() == 4);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExtendedLocalAccelerationTelemetry {
    pub cmd_echo: LocalAccelerationCommand,
}
const _: () = assert!(core::mem::size_of::<ExtendedLocalAccelerationTelemetry>() == 12);

// ─── Pivot ───────────────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PivotDirection {
    #[default]
    Forward  = 0,
    Backward = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct HeadingPivotCommand {
    pub global_theta:           f32,
    pub max_angular_vel:        f32,
    pub max_angular_acc:        f32,
    pub orbit_radius:           f32,
    pub inset_angle:            f32,
    pub direction:              PivotDirection,
    pub compute_inset_angle:    u8,
    pub reserved:               [u8; 2],
}
const _: () = assert!(core::mem::size_of::<HeadingPivotCommand>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct PointPivotCommand {
    pub target_x:               f32,
    pub target_y:               f32,
    pub max_angular_vel:        f32,
    pub max_angular_acc:        f32,
    pub orbit_radius:           f32,
    pub inset_angle:            f32,
    pub direction:              PivotDirection,
    pub compute_inset_angle:    u8,
    pub reserved:               [u8; 2],
}
const _: () = assert!(core::mem::size_of::<PointPivotCommand>() == 28);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct HeadingPivotTelemetry {
    pub reserved: u32,
}
const _: () = assert!(core::mem::size_of::<HeadingPivotTelemetry>() == 4);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PointPivotTelemetry {
    pub reserved: u32,
}
const _: () = assert!(core::mem::size_of::<PointPivotTelemetry>() == 4);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExtendedHeadingPivotTelemetry {
    pub cmd_echo: HeadingPivotCommand,
}
const _: () = assert!(core::mem::size_of::<ExtendedHeadingPivotTelemetry>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExtendedPointPivotTelemetry {
    pub cmd_echo: PointPivotCommand,
}
const _: () = assert!(core::mem::size_of::<ExtendedPointPivotTelemetry>() == 28);

// ─── Line ────────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct HeadingLineCommand {
    pub start_x:                f32,
    pub start_y:                f32,
    pub dir_x:                  f32,
    pub dir_y:                  f32,
    pub line_velocity:          f32,
    pub global_theta:           f32,
    pub max_vel_colinear:       f32,
    pub max_vel_perp:           f32,
    pub max_vel_angular:        f32,
    pub max_accel_colinear:     f32,
    pub max_accel_perp:         f32,
    pub max_accel_angular:      f32,
    pub colinear_start_thresh:  f32,
}
const _: () = assert!(core::mem::size_of::<HeadingLineCommand>() == 52);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct PointLineCommand {
    pub start_x:                f32,
    pub start_y:                f32,
    pub dir_x:                  f32,
    pub dir_y:                  f32,
    pub line_velocity:          f32,
    pub target_x:               f32,
    pub target_y:               f32,
    pub max_vel_colinear:       f32,
    pub max_vel_perp:           f32,
    pub max_vel_angular:        f32,
    pub max_accel_colinear:     f32,
    pub max_accel_perp:         f32,
    pub max_accel_angular:      f32,
    pub colinear_start_thresh:  f32,
}
const _: () = assert!(core::mem::size_of::<PointLineCommand>() == 56);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct HeadingLineTelemetry {
    pub reserved: u32,
}
const _: () = assert!(core::mem::size_of::<HeadingLineTelemetry>() == 4);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PointLineTelemetry {
    pub reserved: u32,
}
const _: () = assert!(core::mem::size_of::<PointLineTelemetry>() == 4);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExtendedHeadingLineTelemetry {
    pub cmd_echo: HeadingLineCommand,
}
const _: () = assert!(core::mem::size_of::<ExtendedHeadingLineTelemetry>() == 52);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExtendedPointLineTelemetry {
    pub cmd_echo: PointLineCommand,
}
const _: () = assert!(core::mem::size_of::<ExtendedPointLineTelemetry>() == 56);

// ─── Vector helpers ──────────────────────────────────────────────────────────

impl GlobalPositionCommand {
    pub fn as_vec3f(&self) -> Vector3<f32> {
        Vector3::new(self.global_x, self.global_y, self.global_theta)
    }
    pub fn from_vec3f(&mut self, v: Vector3<f32>) {
        self.global_x = v.x;
        self.global_y = v.y;
        self.global_theta = v.z;
    }
}

impl GlobalVelocityCommand {
    pub fn as_vec3f(&self) -> Vector3<f32> {
        Vector3::new(self.global_xd, self.global_yd, self.global_omega)
    }
    pub fn from_vec3f(&mut self, v: Vector3<f32>) {
        self.global_xd = v.x;
        self.global_yd = v.y;
        self.global_omega = v.z;
    }
}

impl LocalVelocityCommand {
    pub fn as_vec3f(&self) -> Vector3<f32> {
        Vector3::new(self.local_xd, self.local_yd, self.local_omega)
    }
    pub fn from_vec3f(&mut self, v: Vector3<f32>) {
        self.local_xd = v.x;
        self.local_yd = v.y;
        self.local_omega = v.z;
    }
}

impl GlobalAccelerationCommand {
    pub fn as_vec3f(&self) -> Vector3<f32> {
        Vector3::new(self.global_xdd, self.global_ydd, self.global_alpha)
    }
    pub fn from_vec3f(&mut self, v: Vector3<f32>) {
        self.global_xdd = v.x;
        self.global_ydd = v.y;
        self.global_alpha = v.z;
    }
}

impl LocalAccelerationCommand {
    pub fn as_vec3f(&self) -> Vector3<f32> {
        Vector3::new(self.local_xdd, self.local_ydd, self.local_alpha)
    }
    pub fn from_vec3f(&mut self, v: Vector3<f32>) {
        self.local_xdd = v.x;
        self.local_ydd = v.y;
        self.local_alpha = v.z;
    }
}
