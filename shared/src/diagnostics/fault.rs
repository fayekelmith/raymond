// Fault-domain contracts defining error categories, severity, and latching behavior.
use serde::{Deserialize, Serialize};

use crate::common::time::TimestampMs;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaultSeverity {
	Warning,
	Recoverable,
	Critical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaultCode {
	CommandTimeout,
	EmergencyStopActive,
	MotorDriverFault,
	PowerBrownout,
	SensorFault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultRecord {
	pub code: FaultCode,
	pub severity: FaultSeverity,
	pub latched: bool,
	pub first_seen: TimestampMs,
}
