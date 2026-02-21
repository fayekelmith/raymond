// Runtime motion state machine data used by control and telemetry components.
use serde::{Deserialize, Serialize};

use crate::motion::command::WheelPercent;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmState {
	Disarmed,
	Armed,
	EmergencyStopped,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotionRuntimeState {
	pub arm_state: ArmState,
	pub commanded_wheels: WheelPercent,
	pub emergency_stop_latched: bool,
}

impl Default for MotionRuntimeState {
	fn default() -> Self {
		Self {
			arm_state: ArmState::Disarmed,
			commanded_wheels: WheelPercent::stop(),
			emergency_stop_latched: false,
		}
	}
}
