#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#[cfg(not(test))] // suppress unused import warning
use core::panic::PanicInfo;
mod vga_buffer;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    vga_buffer::Writer::print_something();

    loop {}
}
