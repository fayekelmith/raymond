// Routes shared protocol commands into actuator calls and maintains runtime state.
use shared::common::time::TimestampMs;
use shared::diagnostics::fault::FaultCode;
use shared::motion::command::{SafetyCommand, WheelPercent};
use shared::motion::state::{ArmState, MotionRuntimeState};
use shared::protocol::command::SpineCommand;
use shared::protocol::telemetry::SpineTelemetry;

pub trait MotionActuator {
    fn stop(&mut self);
    fn tank_drive(&mut self, wheels: WheelPercent);
    fn set_armed(&mut self, armed: bool);
}

pub struct NoopActuator;

impl MotionActuator for NoopActuator {
    fn stop(&mut self) {}

    fn tank_drive(&mut self, _wheels: WheelPercent) {}

    fn set_armed(&mut self, _armed: bool) {}
}

pub struct CommandRouter {
    state: MotionRuntimeState,
    active_fault: Option<FaultCode>,
    boot_time: TimestampMs,
    last_command_time: TimestampMs,
}

impl CommandRouter {
    pub fn new(now: TimestampMs) -> Self {
        Self {
            state: MotionRuntimeState::default(),
            active_fault: None,
            boot_time: now,
            last_command_time: now,
        }
    }

    pub fn handle_command<A: MotionActuator>(
        &mut self,
        command: SpineCommand,
        actuator: &mut A,
        now: TimestampMs,
    ) -> SpineTelemetry {
        self.last_command_time = now;

        match command {
            SpineCommand::Stop => {
                self.state.commanded_wheels = WheelPercent::stop();
                actuator.stop();
            }
            SpineCommand::TankDrive(wheels) => {
                if self.state.arm_state == ArmState::Armed && !self.state.emergency_stop_latched {
                    self.state.commanded_wheels = wheels;
                    actuator.tank_drive(wheels);
                } else {
                    self.state.commanded_wheels = WheelPercent::stop();
                    actuator.stop();
                }
            }
            SpineCommand::Safety(safety) => {
                self.handle_safety(safety, actuator);
            }
            SpineCommand::Ping { .. } => {}
        }

        self.telemetry(now)
    }

    pub fn telemetry(&self, now: TimestampMs) -> SpineTelemetry {
        SpineTelemetry {
            timestamp: now,
            motion: self.state,
            uptime_ms: now.as_millis().saturating_sub(self.boot_time.as_millis()),
            last_command_age_ms: now
                .as_millis()
                .saturating_sub(self.last_command_time.as_millis()) as u32,
            active_fault: self.active_fault,
        }
    }

    fn handle_safety<A: MotionActuator>(&mut self, safety: SafetyCommand, actuator: &mut A) {
        match safety {
            SafetyCommand::Arm => {
                if !self.state.emergency_stop_latched {
                    self.state.arm_state = ArmState::Armed;
                    self.active_fault = None;
                    actuator.set_armed(true);
                }
            }
            SafetyCommand::Disarm => {
                self.state.arm_state = ArmState::Disarmed;
                self.state.commanded_wheels = WheelPercent::stop();
                actuator.stop();
                actuator.set_armed(false);
            }
            SafetyCommand::EmergencyStop => {
                self.state.arm_state = ArmState::EmergencyStopped;
                self.state.emergency_stop_latched = true;
                self.state.commanded_wheels = WheelPercent::stop();
                self.active_fault = Some(FaultCode::EmergencyStopActive);
                actuator.stop();
                actuator.set_armed(false);
            }
            SafetyCommand::ClearEmergencyStop => {
                self.state.arm_state = ArmState::Disarmed;
                self.state.emergency_stop_latched = false;
                self.active_fault = None;
                self.state.commanded_wheels = WheelPercent::stop();
                actuator.stop();
                actuator.set_armed(false);
            }
        }
    }
}
