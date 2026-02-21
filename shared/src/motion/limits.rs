// Limit policy helpers for clamping wheel targets and command setpoints.
use serde::{Deserialize, Serialize};

use crate::common::math::clamp;
use crate::motion::command::WheelPercent;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotionLimits {
	pub max_wheel_percent: i8,
}

impl Default for MotionLimits {
	fn default() -> Self {
		Self {
			max_wheel_percent: 100,
		}
	}
}

pub fn clamp_wheel_percent(input: WheelPercent, max_abs: i8) -> WheelPercent {
	let max_abs_f32 = max_abs as f32;
	WheelPercent {
		left: clamp(input.left as f32, -max_abs_f32, max_abs_f32) as i8,
		right: clamp(input.right as f32, -max_abs_f32, max_abs_f32) as i8,
	}
}
