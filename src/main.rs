#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(osdev::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_main]
use core::panic::PanicInfo;
use osdev::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    osdev::test_panic_handler(_info)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    osdev::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();
    println!("no crashes!");

    loop {}
}
