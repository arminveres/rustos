#![no_std] // Disable Standard library
#![no_main] // Since we don't use the default OS entrypoint, we remove main

use core::panic::PanicInfo;

/// This is the panic handler for the os
/// Currently it does not do anything
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO_BUF: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO_BUF.iter().enumerate() {
        // NOTE(aver): it would be more idiomatic, if we abstracted the vga buffer into its own
        // entity.
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
