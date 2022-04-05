#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use fontdue::{Font, FontSettings};

use kernel::{abstractions::FrameBuffer, rendering::{Renderer, backend::cpu::CPURenderer}};
use task::{executor::Executor, Task};

#[cfg(test)] pub mod tests;
#[rustfmt::skip] pub mod constants;

pub mod loaders;
pub mod kernel;
pub mod serial;
pub mod arch;
pub mod task;
pub mod util;

// These functions will call `kernel_main` when done
#[cfg(target_arch = "x86_64")] bootloader::entry_point!(arch::_x86_64::init);

fn kernel_main<F: FrameBuffer>(framebuffer: &mut F) -> ! {
    let mut renderer = Renderer::new(framebuffer, CPURenderer::new());
        
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

    renderer.present();

    renderer.fonts.push(Font::from_bytes(constants::FONT_REGULAR, FontSettings::default()).unwrap());
    // renderer.fonts.push(Font::from_bytes(constants::FONT_NERD_MONO, FontSettings::default()).unwrap());

    renderer.clear_screen();
    renderer.present();

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