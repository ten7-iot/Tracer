//! Flight-safe firmware: flash this before flying.
//!
//! Never touches the radio or enables the SoftDevice, so it cannot transmit.
//! It only cycles the RGB LED (blue -> red -> green) to show it's running.
//! Keep it free of any tracer-lib / GPS / comms code.
//!
//! Build with: cargo xtask flight-safe nrf52840
//! Known-good fallback image: ../../flight-safe/
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let mut red = Output::new(p.P0_26, Level::High, OutputDrive::Standard);
    let mut green = Output::new(p.P0_30, Level::High, OutputDrive::Standard);
    let mut blue = Output::new(p.P0_06, Level::High, OutputDrive::Standard);

    loop {
        for led in [&mut blue, &mut red, &mut green]
        {
            led.set_low();
            Timer::after_millis(300).await;
            led.set_high();
        }
    }
}