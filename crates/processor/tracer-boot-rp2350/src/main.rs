#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Placeholder entry point. Replaced by the real Embassy entry later.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}