// Health telemetry describing liveness and high-level spine runtime status.
use serde::{Deserialize, Serialize};

use crate::common::time::TimestampMs;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthSnapshot {
	pub uptime_ms: u64,
	pub is_armed: bool,
	pub heartbeat_at: TimestampMs,
}
