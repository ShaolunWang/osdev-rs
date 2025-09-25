/*
 * NOTE: you can also disable the harness flag
 * so we don't have to redefine the test runner again!
*/

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osdev::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use osdev::{QemuExitCode, exit_qemu, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[OK]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}

use osdev::serial_print;
#[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail ... \t");
    assert_eq!(0, 1);
}
