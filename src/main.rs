#![no_std] // Not linking the Rust standard library
#![no_main] // Ban all Rust level entries

mod vga_buffer; // VGA I/O support

use core::panic::PanicInfo;

#[no_mangle]
// entry of the operating system
pub extern "C" fn _start() -> ! {
    for i in 0..=100 {
        println!("Loading: percentage={}%", i);
    }
    println!("Permission: level 8.");
    println!("Welcome back, doctor.");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Panicked at {}", _info);
    loop {}
}
