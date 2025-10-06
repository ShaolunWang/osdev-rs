#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(osdev::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_main]
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use osdev::println;
use x86_64::{
    VirtAddr,
    structures::paging::{PageTable, Translate},
};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    osdev::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    osdev::test_panic_handler(_info)
}

entry_point!(kernel_main);

pub fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    osdev::init();
    // x86_64::instructions::interrupts::int3();
    use osdev::memory;
    use x86_64::VirtAddr;

    let physiscal_memory_offset = VirtAddr::new(_boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(physiscal_memory_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        _boot_info.physical_memory_offset,
    ];
    for &addr in &addresses {
        let virt = VirtAddr::new(addr);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("no crashes!");
    osdev::hlt_loop();
}
