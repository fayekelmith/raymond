// Semantic protocol version carried with messages for compatibility checks.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolVersion {
	pub major: u8,
	pub minor: u8,
}

impl ProtocolVersion {
	pub const CURRENT: Self = Self { major: 1, minor: 0 };
}
