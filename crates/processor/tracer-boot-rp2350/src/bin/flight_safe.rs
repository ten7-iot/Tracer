//! Flight-safe firmware: flash this before flying.
//!
//! Must never touch any radio. Keep it free of any tracer-lib / GPS / comms code.
//!
//! PLACEHOLDER: like the main RP2350 binary, this doesn't boot yet. Replace it
//! with a real Embassy LED-blink program once the RP2350 bring-up is done.
//!
//! Build with: cargo xtask flight-safe rp2350
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
