use nalgebra::{Vector2, Vector3};
use crate::{bindings::{BasicControl, BasicTelemetry, BodyControlExtendedTelemetry, BodyControlManeuverExtendedTelemetry, BodyControlMode, BodyControlTelemetry, ExtendedGlobalAccelerationTelemetry, ExtendedGlobalPositionTelemetry, ExtendedGlobalVelocityTelemetry, ExtendedLocalAccelerationTelemetry, ExtendedLocalVelocityTelemetry, ExtendedPivotTelemetry, GlobalAccelerationCommand, GlobalAccelerationTelemetry, GlobalPositionCommand, GlobalPositionTelemetry, GlobalVelocityCommand, GlobalVelocityTelemetry, LocalAccelerationCommand, LocalAccelerationTelemetry, LocalVelocityCommand, LocalVelocityTelemetry, ParameterCommand, PivotCommand}, is_basic_control_packet_safe};

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
    Pivot(PivotCommand),
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
    Pivot(ExtendedPivotTelemetry),
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

impl PivotCommand {
    pub fn center(&self) -> Vector2<f32> {
        Vector2::new(self.global_x_center, self.global_y_center)
    }

    pub fn from_center_and_theta(&mut self, center: Vector2<f32>, theta: f32) {
        self.global_x_center = center.x;
        self.global_y_center = center.y;
        self.global_theta = theta;
    }
}

impl BasicControl {
    pub fn get_maneuver_command(&self) -> ManeuverCommand {
        // union extraction is unsafe
        unsafe {
            match self.body_control_mode {
                BodyControlMode::BCM_OFF => ManeuverCommand::Off,
                BodyControlMode::BCM_GLOBAL_POSITION => ManeuverCommand::GlobalPosition(self.cmd.global_pos),
                BodyControlMode::BCM_GLOBAL_VELOCITY => ManeuverCommand::GlobalVelocity(self.cmd.global_vel),
                BodyControlMode::BCM_LOCAL_VELOCITY => ManeuverCommand::LocalVelocity(self.cmd.local_vel),
                BodyControlMode::BCM_GLOBAL_ACCEL => ManeuverCommand::GlobalAcceleration(self.cmd.global_acc),
                BodyControlMode::BCM_LOCAL_ACCEL => ManeuverCommand::LocalAcceleration(self.cmd.local_acc),
                BodyControlMode::BCM_PIVOT => ManeuverCommand::Pivot(self.cmd.pivot),
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
            ManeuverExtendedTelemetry::Pivot(t) => {
                self.body_control_mode = BodyControlMode::BCM_PIVOT;
                self.maneuver = BodyControlManeuverExtendedTelemetry { pivot: t };
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