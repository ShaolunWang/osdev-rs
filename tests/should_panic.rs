/*
 * NOTE: you can also disable the harness flag
 * so we don't have to redefine the test runner again!
*/

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]

use core::panic::PanicInfo;
use osdev::{QemuExitCode, exit_qemu, serial_print, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();

    serial_println!("[test did not panic]");
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail ... \t");
    assert_eq!(0, 1);
}
