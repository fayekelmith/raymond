#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::{Builder, Config};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// 1. Tell the chip which Interrupt Handler to use for the USB peripheral
bind_interrupts!(struct Irqs{
    USBCTRL_IRQ => InterruptHandler<USB>;
});

// 2. Define the background task
// Runs concurrently with 'main' function
#[embassy_executor::task]
async fn usb_task(mut usb: embassy_usb::UsbDevice<'static, Driver<'static, USB>>) -> ! {
    usb.run().await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // initialize the chip
    let p = embassy_rp::init(Default::default());

    let driver = Driver::new(p.USB, Irqs);

    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Raymond");
    config.product = Some("Spine");
    config.serial_number = Some("007");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Static Buffers for USB Data - STATIC because they'll live in the background task too.
    static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static MSOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();
    static STATE: StaticCell<State> = StaticCell::new();

    let state = STATE.init(State::new());

    let mut builder = Builder::new(
        driver,
        config,
        CONFIG_DESC.init([0; 256]),
        BOS_DESC.init([0; 256]),
        MSOS_DESC.init([0; 256]),
        CONTROL_BUF.init([0; 64]),
    );

    // create the "Serial Port" class

    let mut class = CdcAcmClass::new(&mut builder, state, 64);

    let usb = builder.build();

    //start he background USB task
    spawner.spawn(usb_task(usb)).unwrap();

    loop {
        class.wait_connection().await;

        // send a message

        let _ = class.write_packet(b"Hello from Spine!\r\n").await;

        // sleep for 1 second
        embassy_time::Timer::after_secs(1).await;
    }
}
