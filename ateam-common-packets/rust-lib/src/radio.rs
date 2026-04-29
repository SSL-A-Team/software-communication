use nalgebra::Vector3;
use crate::{bindings::{BasicControl, BasicTelemetry, BodyControlExtendedTelemetry, BodyControlMode, BodyControlSkillExtendedTelemetry, BodyControlTelemetry, ExtendedGlobalAccelerationTelemetry, ExtendedGlobalPositionTelemetry, ExtendedGlobalVelocityTelemetry, ExtendedLocalAccelerationTelemetry, ExtendedLocalVelocityTelemetry, GlobalAccelerationCommand, GlobalAccelerationTelemetry, GlobalPositionCommand, GlobalPositionTelemetry, GlobalVelocityCommand, GlobalVelocityTelemetry, LocalAccelerationCommand, LocalAccelerationTelemetry, LocalVelocityCommand, LocalVelocityTelemetry, ParameterCommand}, is_basic_control_packet_safe};

#[derive(Copy, Clone)]
pub enum DataPacket {
    BasicControl(BasicControl),
    ParameterCommand(ParameterCommand),
}

pub enum SkillCommand {
    Off,
    GlobalPosition(GlobalPositionCommand),
    GlobalVelocity(GlobalVelocityCommand),
    LocalVelocity(LocalVelocityCommand),
    GlobalAcceleration(GlobalAccelerationCommand),
    LocalAcceleration(LocalAccelerationCommand),
}

#[derive(Copy, Clone)]
pub enum SkillTelemetry {
    Off,
    GlobalPosition(GlobalPositionTelemetry),
    GlobalVelocity(GlobalVelocityTelemetry),
    LocalVelocity(LocalVelocityTelemetry),
    GlobalAcceleration(GlobalAccelerationTelemetry),
    LocalAcceleration(LocalAccelerationTelemetry),
}

#[derive(Copy, Clone)]
pub enum SkillExtendedTelemetry {
    Off,
    GlobalPosition(ExtendedGlobalPositionTelemetry),
    GlobalVelocity(ExtendedGlobalVelocityTelemetry),
    LocalVelocity(ExtendedLocalVelocityTelemetry),
    GlobalAcceleration(ExtendedGlobalAccelerationTelemetry),
    LocalAcceleration(ExtendedLocalAccelerationTelemetry),
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
    pub fn get_skill_command(&self) -> SkillCommand {
        // union extraction is unsafe
        unsafe {
            match self.body_control_mode {
                BodyControlMode::BCM_OFF => SkillCommand::Off,
                BodyControlMode::BCM_GLOBAL_POSITION => SkillCommand::GlobalPosition(self.cmd.global_pos),
                BodyControlMode::BCM_GLOBAL_VELOCITY => SkillCommand::GlobalVelocity(self.cmd.global_vel),
                BodyControlMode::BCM_LOCAL_VELOCITY => SkillCommand::LocalVelocity(self.cmd.local_vel),
                BodyControlMode::BCM_GLOBAL_ACCEL => SkillCommand::GlobalAcceleration(self.cmd.global_acc),
                BodyControlMode::BCM_LOCAL_ACCEL => SkillCommand::LocalAcceleration(self.cmd.local_acc),
                _ => SkillCommand::Off,
            }
        }   
    }
}

impl BasicTelemetry {
    pub fn set_skill_telemetry(&mut self, telem: SkillTelemetry) {
        unsafe {
            match telem {
                SkillTelemetry::Off => {
                    self.body_control_mode = BodyControlMode::BCM_OFF;
                }
                SkillTelemetry::GlobalPosition(t) => {
                    self.body_control_mode = BodyControlMode::BCM_GLOBAL_POSITION;
                    self.control_telem = BodyControlTelemetry { global_pos: t };
                }
                SkillTelemetry::GlobalVelocity(t) => {
                    self.body_control_mode = BodyControlMode::BCM_GLOBAL_VELOCITY;
                    self.control_telem = BodyControlTelemetry { global_vel: t };
                }
                SkillTelemetry::LocalVelocity(t) => {
                    self.body_control_mode = BodyControlMode::BCM_LOCAL_VELOCITY;
                    self.control_telem = BodyControlTelemetry { local_vel: t };
                }
                SkillTelemetry::GlobalAcceleration(t) => {
                    self.body_control_mode = BodyControlMode::BCM_GLOBAL_ACCEL;
                    self.control_telem = BodyControlTelemetry { global_acc: t };
                }
                SkillTelemetry::LocalAcceleration(t) => {
                    self.body_control_mode = BodyControlMode::BCM_LOCAL_ACCEL;
                    self.control_telem = BodyControlTelemetry { local_acc: t };
                }
            }
        }
    }
}

impl BodyControlExtendedTelemetry {
    pub fn set_skill_telemetry(&mut self, telem: SkillExtendedTelemetry) {
        unsafe {
            match telem {
                SkillExtendedTelemetry::Off => {
                    self.body_control_mode = BodyControlMode::BCM_OFF;
                }
                SkillExtendedTelemetry::GlobalPosition(t) => {
                    self.body_control_mode = BodyControlMode::BCM_GLOBAL_POSITION;
                    self.skill = BodyControlSkillExtendedTelemetry { global_pos: t };
                }
                SkillExtendedTelemetry::GlobalVelocity(t) => {
                    self.body_control_mode = BodyControlMode::BCM_GLOBAL_VELOCITY;
                    self.skill = BodyControlSkillExtendedTelemetry { global_vel: t };
                }
                SkillExtendedTelemetry::LocalVelocity(t) => {
                    self.body_control_mode = BodyControlMode::BCM_LOCAL_VELOCITY;
                    self.skill = BodyControlSkillExtendedTelemetry { local_vel: t };
                }
                SkillExtendedTelemetry::GlobalAcceleration(t) => {
                    self.body_control_mode = BodyControlMode::BCM_GLOBAL_ACCEL;
                    self.skill = BodyControlSkillExtendedTelemetry { global_acc: t };
                }
                SkillExtendedTelemetry::LocalAcceleration(t) => {
                    self.body_control_mode = BodyControlMode::BCM_LOCAL_ACCEL;
                    self.skill = BodyControlSkillExtendedTelemetry { local_acc: t };
                }
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