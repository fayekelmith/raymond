// HUB75-oriented display contracts that stay independent from PIO driver internals.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmotionPreset {
	Neutral,
	Happy,
	Thinking,
	Warning,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hub75Status {
	pub width: u16,
	pub height: u16,
	pub brightness: u8,
	pub is_enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hub75Command {
	pub preset: EmotionPreset,
	pub brightness: u8,
}
