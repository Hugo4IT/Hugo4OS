#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use memory::{BootInfoFrameAllocator, init_heap, Locked, FixedSizeBlockAllocator};
use bootloader::{entry_point, BootInfo};
use task::{executor::Executor, Task};
use x86_64::VirtAddr;
use core::panic::PanicInfo;

extern crate alloc;

#[cfg(test)] pub mod tests;
#[rustfmt::skip] pub mod constants;

pub mod global_desc_table;
pub mod cpu_renderer;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod utils;
pub mod task;

#[global_allocator] pub static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    global_desc_table::init();
    interrupts::init();

    let physical_memory_offset = boot_info.physical_memory_offset.into_option().unwrap();
    let phys_mem_offset = VirtAddr::new(physical_memory_offset);
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(&boot_info.memory_regions) };
    let mut mapper = unsafe { memory::new_page_table(phys_mem_offset) };

    // Initialize dynamic managed memory
    init_heap(&mut mapper, &mut frame_allocator).unwrap();

    // Graphics drawing, show splash screen
    unsafe { cpu_renderer::init(boot_info.framebuffer.as_mut().unwrap()) };

    let mut executor = Executor::new();
    executor.spawn(Task::new(task::keyboard::print_keypresses()));
    executor.run();
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    utils::hlt_loop();
}

#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    println!("Failed!");
    println!("Error: {}", info);
    tests::exit_qemu(0x10);
    utils::hlt_loop();
}

#[alloc_error_handler]
fn alloc_error(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}