#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
// entry of the operating system
pub extern "C" fn _start() -> ! {
    for i in 0..=10000 {
        println!("{}", i);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
