#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use kernel::memory::{Locked, FixedSizeBlockAllocator};
use task::{executor::Executor, Task};
use core::panic::PanicInfo;

use crate::kernel::rendering::{Renderer, backend::cpu::CPURenderer};

extern crate alloc;

#[cfg(test)] pub mod tests;
#[rustfmt::skip] pub mod constants;

pub mod kernel;
pub mod serial;
pub mod task;

#[global_allocator] pub static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    println_verbose!("Starting Hugo4OS...");
    
    println_verbose!("Kernel");
    println_verbose!("  GDT");
    kernel::gdt::init();
    
    println_verbose!("  Interrupts");
    kernel::interrupts::init();
    kernel::interrupts::disable();
    
    println_verbose!("  Memory management");
    let physical_memory_offset = boot_info.physical_memory_offset.into_option().unwrap();
    kernel::memory::init(physical_memory_offset, &boot_info.memory_regions);
    
    println_verbose!("CPU-based renderer");
    
    let mut renderer = Renderer::new(boot_info.framebuffer.as_mut().unwrap(), CPURenderer::new());

    println_verbose!("  Splash Screen");
        
    // Display splash screen
    
    let center_x = renderer.get_width() / 2;
    let center_y = renderer.get_height() / 2;
    
    let logo_top = center_y - 100;
    let logo_left = center_x - 100;
    let logo_right = logo_left + 200;
    let logo_bottom = logo_top + 200;
    
    renderer.clear_screen();
    
    renderer.fill_rect(logo_left, logo_top, 200, 20, 0xffda0037);
    renderer.fill_rect(logo_left, logo_bottom - 20, 200, 20, 0xffda0037);

    renderer.fill_rect(logo_left + 50, center_y - 10, 100, 20, 0xffd3d3d3);
    renderer.fill_rect(logo_left + 50, logo_top + 40, 20, 120, 0xffd3d3d3);
    renderer.fill_rect(logo_right - 50 - 20, logo_top + 40, 20, 120, 0xffd3d3d3);

    renderer.draw_char(0, 0, 'H', 32.0, 0xffd3d3d3);

    renderer.present();

    println_verbose!("Enable interrupts");
    kernel::interrupts::enable();

    let mut executor = Executor::new();
    executor.spawn(Task::new(task::keyboard::print_keypresses()));
    executor.run();
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    println!("Failed!");
    println!("Error: {}", info);
    tests::exit_qemu(0x10);
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

// I have spent the past 8 F#@$&N HOURS trying to get
// font rasterization to work, but it kept complaining
// about libm and unkown symbols because I guess the
// Rust linker is just complete wack. So I copied
// the "unknown" functions from the libm source over
// here with #[no_mangle], it finally works.

#[no_mangle] pub fn fminf(x: f32, y: f32) -> f32 {
    (if y.is_nan() || x < y { x } else { y }) * 1.0
}

#[no_mangle] pub fn fmaxf(x: f32, y: f32) -> f32 {
    (if x.is_nan() || x < y { y } else { x }) * 1.0
}