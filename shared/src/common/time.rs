// Monotonic timestamp primitives shared across command and telemetry frames.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TimestampMs(pub u64);

impl TimestampMs {
	pub const fn from_millis(value: u64) -> Self {
		Self(value)
	}

	pub const fn as_millis(self) -> u64 {
		self.0
	}
}
