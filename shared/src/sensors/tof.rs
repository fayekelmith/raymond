// Time-of-Flight distance telemetry contracts with quality flags for filtering.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TofQuality {
	Good,
	Saturated,
	OutOfRange,
	SignalTooLow,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TofSample {
	pub distance_mm: u16,
	pub quality: TofQuality,
}
