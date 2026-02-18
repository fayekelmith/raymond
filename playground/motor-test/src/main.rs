#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use motor_test::motor_control;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut front_stby = Output::new(p.PIN_1, Level::Low);
    let mut rear_stby = Output::new(p.PIN_0, Level::Low);

    let mut fl_pwm = Output::new(p.PIN_2, Level::Low);
    let mut fl_in1 = Output::new(p.PIN_3, Level::Low);
    let mut fl_in2 = Output::new(p.PIN_6, Level::Low);

    let mut fr_pwm = Output::new(p.PIN_7, Level::Low);
    let mut fr_in1 = Output::new(p.PIN_10, Level::Low);
    let mut fr_in2 = Output::new(p.PIN_11, Level::Low);

    let mut rl_pwm = Output::new(p.PIN_20, Level::Low);
    let mut rl_in1 = Output::new(p.PIN_21, Level::Low);
    let mut rl_in2 = Output::new(p.PIN_22, Level::Low);

    let mut rr_pwm = Output::new(p.PIN_26, Level::Low);
    let mut rr_in1 = Output::new(p.PIN_27, Level::Low);
    let mut rr_in2 = Output::new(p.PIN_28, Level::Low);

    // Safe startup: brake all, drivers disabled
    motor_control::brake(
        &mut fl_pwm, &mut fl_in1, &mut fl_in2,
        &mut fr_pwm, &mut fr_in1, &mut fr_in2,
        &mut rl_pwm, &mut rl_in1, &mut rl_in2,
        &mut rr_pwm, &mut rr_in1, &mut rr_in2,
    ).await;
    Timer::after_secs(2).await;

    // Enable drivers
    front_stby.set_high();
    rear_stby.set_high();
    Timer::after_millis(500).await;

    loop {
        // === Basic Movement Tests ===
        
        // 1) Forward @ 50%
        motor_control::forward(
            &mut fl_pwm, &mut fl_in1, &mut fl_in2,
            &mut fr_pwm, &mut fr_in1, &mut fr_in2,
            &mut rl_pwm, &mut rl_in1, &mut rl_in2,
            &mut rr_pwm, &mut rr_in1, &mut rr_in2,
            50, Duration::from_secs(3)
        ).await;
        Timer::after_secs(1).await;

        // 2) Reverse @ 50%
        motor_control::reverse(
            &mut fl_pwm, &mut fl_in1, &mut fl_in2,
            &mut fr_pwm, &mut fr_in1, &mut fr_in2,
            &mut rl_pwm, &mut rl_in1, &mut rl_in2,
            &mut rr_pwm, &mut rr_in1, &mut rr_in2,
            50, Duration::from_secs(3)
        ).await;
        Timer::after_secs(2).await;

        // === Arc Turn Tests (smooth curves) ===
        
        // 3) Gentle arc right (left=60%, right=30%)
        motor_control::arc_right(
            &mut fl_pwm, &mut fl_in1, &mut fl_in2,
            &mut fr_pwm, &mut fr_in1, &mut fr_in2,
            &mut rl_pwm, &mut rl_in1, &mut rl_in2,
            &mut rr_pwm, &mut rr_in1, &mut rr_in2,
            60, 30, Duration::from_secs(4)
        ).await;
        Timer::after_secs(1).await;

        // 4) Gentle arc left (right=60%, left=30%)
        motor_control::arc_left(
            &mut fl_pwm, &mut fl_in1, &mut fl_in2,
            &mut fr_pwm, &mut fr_in1, &mut fr_in2,
            &mut rl_pwm, &mut rl_in1, &mut rl_in2,
            &mut rr_pwm, &mut rr_in1, &mut rr_in2,
            60, 30, Duration::from_secs(4)
        ).await;
        Timer::after_secs(2).await;

        // === Pivot Turn Tests (sharp turns) ===
        
        // 5) Pivot right (left wheels only @ 50%)
        motor_control::pivot_right(
            &mut fl_pwm, &mut fl_in1, &mut fl_in2,
            &mut fr_pwm, &mut fr_in1, &mut fr_in2,
            &mut rl_pwm, &mut rl_in1, &mut rl_in2,
            &mut rr_pwm, &mut rr_in1, &mut rr_in2,
            50, Duration::from_secs(3)
        ).await;
        Timer::after_secs(1).await;

        // 6) Pivot left (right wheels only @ 50%)
        motor_control::pivot_left(
            &mut fl_pwm, &mut fl_in1, &mut fl_in2,
            &mut fr_pwm, &mut fr_in1, &mut fr_in2,
            &mut rl_pwm, &mut rl_in1, &mut rl_in2,
            &mut rr_pwm, &mut rr_in1, &mut rr_in2,
            50, Duration::from_secs(3)
        ).await;
        Timer::after_secs(2).await;

        // === Spin Tests (zero-radius rotation) ===
        
        // 7) Spin clockwise (left forward, right reverse @ 40%)
        motor_control::spin_clockwise(
            &mut fl_pwm, &mut fl_in1, &mut fl_in2,
            &mut fr_pwm, &mut fr_in1, &mut fr_in2,
            &mut rl_pwm, &mut rl_in1, &mut rl_in2,
            &mut rr_pwm, &mut rr_in1, &mut rr_in2,
            40, Duration::from_secs(3)
        ).await;
        Timer::after_secs(1).await;

        // 8) Spin counter-clockwise (left reverse, right forward @ 40%)
        motor_control::spin_counterclockwise(
            &mut fl_pwm, &mut fl_in1, &mut fl_in2,
            &mut fr_pwm, &mut fr_in1, &mut fr_in2,
            &mut rl_pwm, &mut rl_in1, &mut rl_in2,
            &mut rr_pwm, &mut rr_in1, &mut rr_in2,
            40, Duration::from_secs(3)
        ).await;
        Timer::after_secs(2).await;

        // === Emergency Stop Test ===
        
        // 9) Simulate emergency stop
        motor_control::brake(
            &mut fl_pwm, &mut fl_in1, &mut fl_in2,
            &mut fr_pwm, &mut fr_in1, &mut fr_in2,
            &mut rl_pwm, &mut rl_in1, &mut rl_in2,
            &mut rr_pwm, &mut rr_in1, &mut rr_in2,
        ).await;
        front_stby.set_low();
        rear_stby.set_low();
        Timer::after_secs(2).await;
        
        // Re-enable
        front_stby.set_high();
        rear_stby.set_high();
        Timer::after_secs(1).await;
    }
}

