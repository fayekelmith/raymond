#![no_std]
#![no_main]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());

    defmt::println!("Hello, from embedded world!");

    // Example use of shared library
    let sum = shared::add(5, 7);
    defmt::println!("5 + 7 = {}", sum);
}
