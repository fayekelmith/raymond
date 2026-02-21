// IMU telemetry contracts independent of any specific sensor driver implementation.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
	pub w: f32,
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EulerAngles {
	pub roll_rad: f32,
	pub pitch_rad: f32,
	pub yaw_rad: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImuSample {
	pub orientation_quat: Quaternion,
	pub orientation_euler: EulerAngles,
}
