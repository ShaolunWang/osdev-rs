#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    vga_buffer::WRITER.lock().write_str("hello world").unwrap();
    write!(vga_buffer::WRITER.lock(), ", number: {}", 42).unwrap();
    loop {}
}
