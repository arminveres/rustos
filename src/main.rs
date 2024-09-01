#![no_std] // Disable Standard library
#![no_main] // Since we don't use the default OS entrypoint, we remove main

use core::panic::PanicInfo;

/// This is the panic handler for the os
/// Currently it does not do anything
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
/// .
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
