#![no_std] // Don't link the Rust standard library 
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // Don't mangle the name of this function 
// ! means not allowed to ever return 
// Entry point invoked by the os or bootloader
pub extern "C" fn _start() -> ! {
    // The linker looks for a function named `_start` by default 
    loop {}
}

// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
