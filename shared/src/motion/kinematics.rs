// Hardware-agnostic kinematics helpers converting throttle/turn into wheel percentages.
use crate::motion::command::WheelPercent;
use crate::motion::limits::clamp_wheel_percent;

pub fn differential_mix_percent(throttle: i8, turn: i8) -> WheelPercent {
	let left = throttle as i16 + turn as i16;
	let right = throttle as i16 - turn as i16;
	let left = left.clamp(-100, 100);
	let right = right.clamp(-100, 100);

	clamp_wheel_percent(
		WheelPercent {
			left: left as i8,
			right: right as i8,
		},
		100,
	)
}
