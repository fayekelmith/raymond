// High-level motion and safety intents independent from board-level motor drivers.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WheelPercent {
	pub left: i8,
	pub right: i8,
}

impl WheelPercent {
	pub const fn stop() -> Self {
		Self { left: 0, right: 0 }
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyCommand {
	Arm,
	Disarm,
	EmergencyStop,
	ClearEmergencyStop,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MotionCommand {
	Stop,
	TankDrive(WheelPercent),
}
