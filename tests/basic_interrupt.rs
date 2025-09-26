#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osdev::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use osdev::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osdev::test_panic_handler(info)
}

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    osdev::init();
    test_main();
    loop {}
}
#[test_case]
fn test_interrupts() {
    x86_64::instructions::interrupts::int3();
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
