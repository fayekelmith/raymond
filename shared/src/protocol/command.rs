// Protocol command message types exchanged with spine control firmware.
use serde::{Deserialize, Serialize};

use crate::common::time::TimestampMs;
use crate::motion::command::{SafetyCommand, WheelPercent};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SpineCommand {
	Stop,
	TankDrive(WheelPercent),
	Safety(SafetyCommand),
	Ping { sent_at: TimestampMs },
}
