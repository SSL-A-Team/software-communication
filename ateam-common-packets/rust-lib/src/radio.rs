use nalgebra::Vector3;
use crate::{bindings::{BasicControl, BasicTelemetry, BodyControlExtendedTelemetry, BodyControlManeuverExtendedTelemetry, BodyControlMode, BodyControlTelemetry, ExtendedGlobalAccelerationTelemetry, ExtendedGlobalPositionTelemetry, ExtendedGlobalVelocityTelemetry, ExtendedHeadingPivotTelemetry, ExtendedHeadingLineTelemetry, ExtendedLocalAccelerationTelemetry, ExtendedLocalVelocityTelemetry, ExtendedPointPivotTelemetry, ExtendedPointLineTelemetry, GlobalAccelerationCommand, GlobalAccelerationTelemetry, GlobalPositionCommand, GlobalPositionTelemetry, GlobalVelocityCommand, GlobalVelocityTelemetry, HeadingPivotCommand, HeadingLineCommand, LocalAccelerationCommand, LocalAccelerationTelemetry, LocalVelocityCommand, LocalVelocityTelemetry, ParameterCommand, PointPivotCommand, PointLineCommand}, is_basic_control_packet_safe};

#[derive(Copy, Clone)]
pub enum DataPacket {
    BasicControl(BasicControl),
    ParameterCommand(ParameterCommand),
}

pub enum ManeuverCommand {
    Off,
    GlobalPosition(GlobalPositionCommand),
    GlobalVelocity(GlobalVelocityCommand),
    LocalVelocity(LocalVelocityCommand),
    GlobalAcceleration(GlobalAccelerationCommand),
    LocalAcceleration(LocalAccelerationCommand),
    HeadingPivot(HeadingPivotCommand),
    PointPivot(PointPivotCommand),
    HeadingLine(HeadingLineCommand),
    PointLine(PointLineCommand),
}

impl Copy for ManeuverCommand {}

impl Clone for ManeuverCommand {
    fn clone(&self) -> Self {
        *self
    }
}

impl PartialEq for ManeuverCommand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Off, Self::Off) => true,
            (Self::GlobalPosition(a), Self::GlobalPosition(b)) => {
                a.global_x == b.global_x
                    && a.global_y == b.global_y
                    && a.global_theta == b.global_theta
                    && a.max_linear_vel == b.max_linear_vel
                    && a.max_angular_vel == b.max_angular_vel
                    && a.max_linear_acc == b.max_linear_acc
                    && a.max_angular_acc == b.max_angular_acc
            }
            (Self::GlobalVelocity(a), Self::GlobalVelocity(b)) => {
                a.global_xd == b.global_xd
                    && a.global_yd == b.global_yd
                    && a.global_omega == b.global_omega
                    && a.max_linear_acc == b.max_linear_acc
                    && a.max_angular_acc == b.max_angular_acc
            }
            (Self::LocalVelocity(a), Self::LocalVelocity(b)) => {
                a.local_xd == b.local_xd
                    && a.local_yd == b.local_yd
                    && a.local_omega == b.local_omega
                    && a.max_linear_acc == b.max_linear_acc
                    && a.max_angular_acc == b.max_angular_acc
            }
            (Self::GlobalAcceleration(a), Self::GlobalAcceleration(b)) => {
                a.global_xdd == b.global_xdd
                    && a.global_ydd == b.global_ydd
                    && a.global_alpha == b.global_alpha
            }
            (Self::LocalAcceleration(a), Self::LocalAcceleration(b)) => {
                a.local_xdd == b.local_xdd
                    && a.local_ydd == b.local_ydd
                    && a.local_alpha == b.local_alpha
            }
            (Self::HeadingPivot(a), Self::HeadingPivot(b)) => {
                a.global_theta == b.global_theta
                    && a.max_angular_vel == b.max_angular_vel
                    && a.max_angular_acc == b.max_angular_acc
                    && a.orbit_radius == b.orbit_radius
                    && a.inset_angle == b.inset_angle
                    && a.direction == b.direction
                    && a.compute_inset_angle == b.compute_inset_angle
            }
            (Self::PointPivot(a), Self::PointPivot(b)) => {
                a.target_x == b.target_x
                    && a.target_y == b.target_y
                    && a.max_angular_vel == b.max_angular_vel
                    && a.max_angular_acc == b.max_angular_acc
                    && a.orbit_radius == b.orbit_radius
                    && a.inset_angle == b.inset_angle
                    && a.direction == b.direction
                    && a.compute_inset_angle == b.compute_inset_angle
            }
            (Self::HeadingLine(a), Self::HeadingLine(b)) => {
                a.start_x == b.start_x
                    && a.start_y == b.start_y
                    && a.dir_x == b.dir_x
                    && a.dir_y == b.dir_y
                    && a.line_velocity == b.line_velocity
                    && a.global_theta == b.global_theta
                    && a.max_vel_colinear == b.max_vel_colinear
                    && a.max_vel_perp == b.max_vel_perp
                    && a.max_vel_angular == b.max_vel_angular
                    && a.max_accel_colinear == b.max_accel_colinear
                    && a.max_accel_perp == b.max_accel_perp
                    && a.max_accel_angular == b.max_accel_angular
                    && a.colinear_start_thresh == b.colinear_start_thresh
            }
            (Self::PointLine(a), Self::PointLine(b)) => {
                a.start_x == b.start_x
                    && a.start_y == b.start_y
                    && a.dir_x == b.dir_x
                    && a.dir_y == b.dir_y
                    && a.line_velocity == b.line_velocity
                    && a.target_x == b.target_x
                    && a.target_y == b.target_y
                    && a.max_vel_colinear == b.max_vel_colinear
                    && a.max_vel_perp == b.max_vel_perp
                    && a.max_vel_angular == b.max_vel_angular
                    && a.max_accel_colinear == b.max_accel_colinear
                    && a.max_accel_perp == b.max_accel_perp
                    && a.max_accel_angular == b.max_accel_angular
                    && a.colinear_start_thresh == b.colinear_start_thresh
            }
            _ => false,
        }
    }
}

#[derive(Copy, Clone)]
pub enum ManeuverTelemetry {
    Off,
    GlobalPosition(GlobalPositionTelemetry),
    GlobalVelocity(GlobalVelocityTelemetry),
    LocalVelocity(LocalVelocityTelemetry),
    GlobalAcceleration(GlobalAccelerationTelemetry),
    LocalAcceleration(LocalAccelerationTelemetry),
}

#[derive(Copy, Clone)]
pub enum ManeuverExtendedTelemetry {
    Off,
    GlobalPosition(ExtendedGlobalPositionTelemetry),
    GlobalVelocity(ExtendedGlobalVelocityTelemetry),
    LocalVelocity(ExtendedLocalVelocityTelemetry),
    GlobalAcceleration(ExtendedGlobalAccelerationTelemetry),
    LocalAcceleration(ExtendedLocalAccelerationTelemetry),
    HeadingPivot(ExtendedHeadingPivotTelemetry),
    PointPivot(ExtendedPointPivotTelemetry),
    HeadingLine(ExtendedHeadingLineTelemetry),
    PointLine(ExtendedPointLineTelemetry),
}

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

impl BasicControl {
    pub fn get_maneuver_command(&self) -> ManeuverCommand {
        // SAFETY: body_control_mode is the discriminant written alongside the union;
        // we only access the arm that matches the discriminant value.
        unsafe {
            match self.body_control_mode {
                BodyControlMode::BCM_OFF => ManeuverCommand::Off,
                BodyControlMode::BCM_GLOBAL_POSITION => ManeuverCommand::GlobalPosition(self.cmd.global_pos),
                BodyControlMode::BCM_GLOBAL_VELOCITY => ManeuverCommand::GlobalVelocity(self.cmd.global_vel),
                BodyControlMode::BCM_LOCAL_VELOCITY => ManeuverCommand::LocalVelocity(self.cmd.local_vel),
                BodyControlMode::BCM_GLOBAL_ACCEL => ManeuverCommand::GlobalAcceleration(self.cmd.global_acc),
                BodyControlMode::BCM_LOCAL_ACCEL => ManeuverCommand::LocalAcceleration(self.cmd.local_acc),
                BodyControlMode::BCM_HEADING_PIVOT => ManeuverCommand::HeadingPivot(self.cmd.heading_pivot),
                BodyControlMode::BCM_POINT_PIVOT => ManeuverCommand::PointPivot(self.cmd.point_pivot),
                BodyControlMode::BCM_HEADING_LINE => ManeuverCommand::HeadingLine(self.cmd.heading_line),
                BodyControlMode::BCM_POINT_LINE => ManeuverCommand::PointLine(self.cmd.point_line),
                _ => ManeuverCommand::Off,
            }
        }   
    }
}

impl BasicTelemetry {
    pub fn set_maneuver_telemetry(&mut self, telem: ManeuverTelemetry) {
        match telem {
            ManeuverTelemetry::Off => {
                self.body_control_mode = BodyControlMode::BCM_OFF;
            }
            ManeuverTelemetry::GlobalPosition(t) => {
                self.body_control_mode = BodyControlMode::BCM_GLOBAL_POSITION;
                self.control_telem = BodyControlTelemetry { global_pos: t };
            }
            ManeuverTelemetry::GlobalVelocity(t) => {
                self.body_control_mode = BodyControlMode::BCM_GLOBAL_VELOCITY;
                self.control_telem = BodyControlTelemetry { global_vel: t };
            }
            ManeuverTelemetry::LocalVelocity(t) => {
                self.body_control_mode = BodyControlMode::BCM_LOCAL_VELOCITY;
                self.control_telem = BodyControlTelemetry { local_vel: t };
            }
            ManeuverTelemetry::GlobalAcceleration(t) => {
                self.body_control_mode = BodyControlMode::BCM_GLOBAL_ACCEL;
                self.control_telem = BodyControlTelemetry { global_acc: t };
            }
            ManeuverTelemetry::LocalAcceleration(t) => {
                self.body_control_mode = BodyControlMode::BCM_LOCAL_ACCEL;
                self.control_telem = BodyControlTelemetry { local_acc: t };
            }
        }
    }
}

impl BodyControlExtendedTelemetry {
    pub fn set_maneuver_telemetry(&mut self, telem: ManeuverExtendedTelemetry) {
        match telem {
            ManeuverExtendedTelemetry::Off => {
                self.body_control_mode = BodyControlMode::BCM_OFF;
            }
            ManeuverExtendedTelemetry::GlobalPosition(t) => {
                self.body_control_mode = BodyControlMode::BCM_GLOBAL_POSITION;
                self.maneuver = BodyControlManeuverExtendedTelemetry { global_pos: t };
            }
            ManeuverExtendedTelemetry::GlobalVelocity(t) => {
                self.body_control_mode = BodyControlMode::BCM_GLOBAL_VELOCITY;
                self.maneuver = BodyControlManeuverExtendedTelemetry { global_vel: t };
            }
            ManeuverExtendedTelemetry::LocalVelocity(t) => {
                self.body_control_mode = BodyControlMode::BCM_LOCAL_VELOCITY;
                self.maneuver = BodyControlManeuverExtendedTelemetry { local_vel: t };
            }
            ManeuverExtendedTelemetry::GlobalAcceleration(t) => {
                self.body_control_mode = BodyControlMode::BCM_GLOBAL_ACCEL;
                self.maneuver = BodyControlManeuverExtendedTelemetry { global_acc: t };
            }
            ManeuverExtendedTelemetry::LocalAcceleration(t) => {
                self.body_control_mode = BodyControlMode::BCM_LOCAL_ACCEL;
                self.maneuver = BodyControlManeuverExtendedTelemetry { local_acc: t };
            }
            ManeuverExtendedTelemetry::HeadingPivot(t) => {
                self.body_control_mode = BodyControlMode::BCM_HEADING_PIVOT;
                self.maneuver = BodyControlManeuverExtendedTelemetry { heading_pivot: t };
            }
            ManeuverExtendedTelemetry::PointPivot(t) => {
                self.body_control_mode = BodyControlMode::BCM_POINT_PIVOT;
                self.maneuver = BodyControlManeuverExtendedTelemetry { point_pivot: t };
            }
            ManeuverExtendedTelemetry::HeadingLine(t) => {
                self.body_control_mode = BodyControlMode::BCM_HEADING_LINE;
                self.maneuver = BodyControlManeuverExtendedTelemetry { heading_line: t };
            }
            ManeuverExtendedTelemetry::PointLine(t) => {
                self.body_control_mode = BodyControlMode::BCM_POINT_LINE;
                self.maneuver = BodyControlManeuverExtendedTelemetry { point_line: t };
            }
        }
    }
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