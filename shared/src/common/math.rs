// Small math helpers for deterministic clamping, deadband, and linear mapping.
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
	if value < min {
		min
	} else if value > max {
		max
	} else {
		value
	}
}

pub fn apply_deadband(value: f32, threshold: f32) -> f32 {
	if value.abs() < threshold {
		0.0
	} else {
		value
	}
}

pub fn map_linear(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
	let scale = (value - in_min) / (in_max - in_min);
	out_min + scale * (out_max - out_min)
}
