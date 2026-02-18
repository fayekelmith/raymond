use embassy_rp::gpio::Output;
use embassy_time::{Duration, Instant, Timer};

#[derive(Copy, Clone)]
pub enum Direction {
    Forward,
    Reverse,
    Brake,
}

fn set_direction(in1: &mut Output<'_>, in2: &mut Output<'_>, direction: Direction) {
    match direction {
        Direction::Forward => {
            in1.set_low();
            in2.set_high();
        }
        Direction::Reverse => {
            in1.set_high();
            in2.set_low();
        }
        Direction::Brake => {
            in1.set_low();
            in2.set_low();
        }
    }
}

async fn run_motors(
    fl_pwm: &mut Output<'_>,
    fl_in1: &mut Output<'_>,
    fl_in2: &mut Output<'_>,
    fr_pwm: &mut Output<'_>,
    fr_in1: &mut Output<'_>,
    fr_in2: &mut Output<'_>,
    rl_pwm: &mut Output<'_>,
    rl_in1: &mut Output<'_>,
    rl_in2: &mut Output<'_>,
    rr_pwm: &mut Output<'_>,
    rr_in1: &mut Output<'_>,
    rr_in2: &mut Output<'_>,
    fl_speed: u8,
    fr_speed: u8,
    rl_speed: u8,
    rr_speed: u8,
    fl_dir: Direction,
    fr_dir: Direction,
    rl_dir: Direction,
    rr_dir: Direction,
    duration: Duration,
) {
    set_direction(fl_in1, fl_in2, fl_dir);
    set_direction(fr_in1, fr_in2, fr_dir);
    set_direction(rl_in1, rl_in2, rl_dir);
    set_direction(rr_in1, rr_in2, rr_dir);

    let period_ms: u64 = 20;
    let end = Instant::now() + duration;

    while Instant::now() < end {
        // Calculate on-times
        let fl_on = (period_ms * fl_speed as u64) / 100;
        let fr_on = (period_ms * fr_speed as u64) / 100;
        let rl_on = (period_ms * rl_speed as u64) / 100;
        let rr_on = (period_ms * rr_speed as u64) / 100;
        let max_on = fl_on.max(fr_on).max(rl_on).max(rr_on);

        // Set all high initially
        if fl_speed > 0 { fl_pwm.set_high(); }
        if fr_speed > 0 { fr_pwm.set_high(); }
        if rl_speed > 0 { rl_pwm.set_high(); }
        if rr_speed > 0 { rr_pwm.set_high(); }

        // Turn off at appropriate times
        for elapsed in 0..=max_on {
            if elapsed == fl_on { fl_pwm.set_low(); }
            if elapsed == fr_on { fr_pwm.set_low(); }
            if elapsed == rl_on { rl_pwm.set_low(); }
            if elapsed == rr_on { rr_pwm.set_low(); }
            
            if elapsed < max_on {
                Timer::after_millis(1).await;
            }
        }

        // Off period
        let off_ms = period_ms.saturating_sub(max_on);
        if off_ms > 0 {
            Timer::after_millis(off_ms).await;
        }
    }

    // Ensure all PWM off
    fl_pwm.set_low();
    fr_pwm.set_low();
    rl_pwm.set_low();
    rr_pwm.set_low();
}

/// Move forward with all wheels at the same speed
pub async fn forward(
    fl_pwm: &mut Output<'_>, fl_in1: &mut Output<'_>, fl_in2: &mut Output<'_>,
    fr_pwm: &mut Output<'_>, fr_in1: &mut Output<'_>, fr_in2: &mut Output<'_>,
    rl_pwm: &mut Output<'_>, rl_in1: &mut Output<'_>, rl_in2: &mut Output<'_>,
    rr_pwm: &mut Output<'_>, rr_in1: &mut Output<'_>, rr_in2: &mut Output<'_>,
    speed: u8,
    duration: Duration,
) {
    run_motors(
        fl_pwm, fl_in1, fl_in2, fr_pwm, fr_in1, fr_in2,
        rl_pwm, rl_in1, rl_in2, rr_pwm, rr_in1, rr_in2,
        speed, speed, speed, speed,
        Direction::Forward, Direction::Forward, Direction::Forward, Direction::Forward,
        duration,
    )
    .await;
}

/// Move backward with all wheels at the same speed
pub async fn reverse(
    fl_pwm: &mut Output<'_>, fl_in1: &mut Output<'_>, fl_in2: &mut Output<'_>,
    fr_pwm: &mut Output<'_>, fr_in1: &mut Output<'_>, fr_in2: &mut Output<'_>,
    rl_pwm: &mut Output<'_>, rl_in1: &mut Output<'_>, rl_in2: &mut Output<'_>,
    rr_pwm: &mut Output<'_>, rr_in1: &mut Output<'_>, rr_in2: &mut Output<'_>,
    speed: u8,
    duration: Duration,
) {
    run_motors(
        fl_pwm, fl_in1, fl_in2, fr_pwm, fr_in1, fr_in2,
        rl_pwm, rl_in1, rl_in2, rr_pwm, rr_in1, rr_in2,
        speed, speed, speed, speed,
        Direction::Reverse, Direction::Reverse, Direction::Reverse, Direction::Reverse,
        duration,
    )
    .await;
}

/// Gentle arc turn right (left wheels faster than right)
pub async fn arc_right(
    fl_pwm: &mut Output<'_>, fl_in1: &mut Output<'_>, fl_in2: &mut Output<'_>,
    fr_pwm: &mut Output<'_>, fr_in1: &mut Output<'_>, fr_in2: &mut Output<'_>,
    rl_pwm: &mut Output<'_>, rl_in1: &mut Output<'_>, rl_in2: &mut Output<'_>,
    rr_pwm: &mut Output<'_>, rr_in1: &mut Output<'_>, rr_in2: &mut Output<'_>,
    left_speed: u8,
    right_speed: u8,
    duration: Duration,
) {
    run_motors(
        fl_pwm, fl_in1, fl_in2, fr_pwm, fr_in1, fr_in2,
        rl_pwm, rl_in1, rl_in2, rr_pwm, rr_in1, rr_in2,
        left_speed, right_speed, left_speed, right_speed,
        Direction::Forward, Direction::Forward, Direction::Forward, Direction::Forward,
        duration,
    )
    .await;
}

/// Gentle arc turn left (right wheels faster than left)
pub async fn arc_left(
    fl_pwm: &mut Output<'_>, fl_in1: &mut Output<'_>, fl_in2: &mut Output<'_>,
    fr_pwm: &mut Output<'_>, fr_in1: &mut Output<'_>, fr_in2: &mut Output<'_>,
    rl_pwm: &mut Output<'_>, rl_in1: &mut Output<'_>, rl_in2: &mut Output<'_>,
    rr_pwm: &mut Output<'_>, rr_in1: &mut Output<'_>, rr_in2: &mut Output<'_>,
    right_speed: u8,
    left_speed: u8,
    duration: Duration,
) {
    run_motors(
        fl_pwm, fl_in1, fl_in2, fr_pwm, fr_in1, fr_in2,
        rl_pwm, rl_in1, rl_in2, rr_pwm, rr_in1, rr_in2,
        left_speed, right_speed, left_speed, right_speed,
        Direction::Forward, Direction::Forward, Direction::Forward, Direction::Forward,
        duration,
    )
    .await;
}

/// Sharp pivot right (left wheels forward, right wheels stopped)
pub async fn pivot_right(
    fl_pwm: &mut Output<'_>, fl_in1: &mut Output<'_>, fl_in2: &mut Output<'_>,
    fr_pwm: &mut Output<'_>, fr_in1: &mut Output<'_>, fr_in2: &mut Output<'_>,
    rl_pwm: &mut Output<'_>, rl_in1: &mut Output<'_>, rl_in2: &mut Output<'_>,
    rr_pwm: &mut Output<'_>, rr_in1: &mut Output<'_>, rr_in2: &mut Output<'_>,
    speed: u8,
    duration: Duration,
) {
    run_motors(
        fl_pwm, fl_in1, fl_in2, fr_pwm, fr_in1, fr_in2,
        rl_pwm, rl_in1, rl_in2, rr_pwm, rr_in1, rr_in2,
        speed, 0, speed, 0,
        Direction::Forward, Direction::Brake, Direction::Forward, Direction::Brake,
        duration,
    )
    .await;
}

/// Sharp pivot left (right wheels forward, left wheels stopped)
pub async fn pivot_left(
    fl_pwm: &mut Output<'_>, fl_in1: &mut Output<'_>, fl_in2: &mut Output<'_>,
    fr_pwm: &mut Output<'_>, fr_in1: &mut Output<'_>, fr_in2: &mut Output<'_>,
    rl_pwm: &mut Output<'_>, rl_in1: &mut Output<'_>, rl_in2: &mut Output<'_>,
    rr_pwm: &mut Output<'_>, rr_in1: &mut Output<'_>, rr_in2: &mut Output<'_>,
    speed: u8,
    duration: Duration,
) {
    run_motors(
        fl_pwm, fl_in1, fl_in2, fr_pwm, fr_in1, fr_in2,
        rl_pwm, rl_in1, rl_in2, rr_pwm, rr_in1, rr_in2,
        0, speed, 0, speed,
        Direction::Brake, Direction::Forward, Direction::Brake, Direction::Forward,
        duration,
    )
    .await;
}

/// Zero-radius spin clockwise (left forward, right reverse)
pub async fn spin_clockwise(
    fl_pwm: &mut Output<'_>, fl_in1: &mut Output<'_>, fl_in2: &mut Output<'_>,
    fr_pwm: &mut Output<'_>, fr_in1: &mut Output<'_>, fr_in2: &mut Output<'_>,
    rl_pwm: &mut Output<'_>, rl_in1: &mut Output<'_>, rl_in2: &mut Output<'_>,
    rr_pwm: &mut Output<'_>, rr_in1: &mut Output<'_>, rr_in2: &mut Output<'_>,
    speed: u8,
    duration: Duration,
) {
    run_motors(
        fl_pwm, fl_in1, fl_in2, fr_pwm, fr_in1, fr_in2,
        rl_pwm, rl_in1, rl_in2, rr_pwm, rr_in1, rr_in2,
        speed, speed, speed, speed,
        Direction::Forward, Direction::Reverse, Direction::Forward, Direction::Reverse,
        duration,
    )
    .await;
}

/// Zero-radius spin counter-clockwise (left reverse, right forward)
pub async fn spin_counterclockwise(
    fl_pwm: &mut Output<'_>, fl_in1: &mut Output<'_>, fl_in2: &mut Output<'_>,
    fr_pwm: &mut Output<'_>, fr_in1: &mut Output<'_>, fr_in2: &mut Output<'_>,
    rl_pwm: &mut Output<'_>, rl_in1: &mut Output<'_>, rl_in2: &mut Output<'_>,
    rr_pwm: &mut Output<'_>, rr_in1: &mut Output<'_>, rr_in2: &mut Output<'_>,
    speed: u8,
    duration: Duration,
) {
    run_motors(
        fl_pwm, fl_in1, fl_in2, fr_pwm, fr_in1, fr_in2,
        rl_pwm, rl_in1, rl_in2, rr_pwm, rr_in1, rr_in2,
        speed, speed, speed, speed,
        Direction::Reverse, Direction::Forward, Direction::Reverse, Direction::Forward,
        duration,
    )
    .await;
}

/// Stop all motors (brake)
pub async fn brake(
    fl_pwm: &mut Output<'_>, fl_in1: &mut Output<'_>, fl_in2: &mut Output<'_>,
    fr_pwm: &mut Output<'_>, fr_in1: &mut Output<'_>, fr_in2: &mut Output<'_>,
    rl_pwm: &mut Output<'_>, rl_in1: &mut Output<'_>, rl_in2: &mut Output<'_>,
    rr_pwm: &mut Output<'_>, rr_in1: &mut Output<'_>, rr_in2: &mut Output<'_>,
) {
    set_direction(fl_in1, fl_in2, Direction::Brake);
    set_direction(fr_in1, fr_in2, Direction::Brake);
    set_direction(rl_in1, rl_in2, Direction::Brake);
    set_direction(rr_in1, rr_in2, Direction::Brake);
    fl_pwm.set_low();
    fr_pwm.set_low();
    rl_pwm.set_low();
    rr_pwm.set_low();
}
