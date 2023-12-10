#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
// entry of the operating system
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    for i in 'a'..'z' {
        write!(vga_buffer::WRITER.lock(), "{}", i).unwrap()
    }
    // write!(vga_buffer::WRITER.lock(), "{} Hello World", '\n').unwrap();
    // write!(vga_buffer::WRITER.lock(), "114514 numbers: {}", 1919810).unwrap();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
