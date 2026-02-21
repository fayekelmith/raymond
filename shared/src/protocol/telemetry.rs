// Protocol telemetry payloads published by spine for downstream consumers.
use serde::{Deserialize, Serialize};

use crate::common::time::TimestampMs;
use crate::diagnostics::fault::FaultCode;
use crate::motion::state::MotionRuntimeState;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpineTelemetry {
	pub timestamp: TimestampMs,
	pub motion: MotionRuntimeState,
	pub uptime_ms: u64,
	pub last_command_age_ms: u32,
	pub active_fault: Option<FaultCode>,
}
