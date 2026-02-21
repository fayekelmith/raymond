// Wire envelope metadata for routing, sequencing, and payload integrity checks.
use serde::{Deserialize, Serialize};

use crate::protocol::version::ProtocolVersion;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageKind {
	Command,
	Telemetry,
	Ack,
	Nack,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageEnvelope {
	pub version: ProtocolVersion,
	pub kind: MessageKind,
	pub sequence: u32,
	pub payload_len: u16,
	pub crc16: u16,
}
