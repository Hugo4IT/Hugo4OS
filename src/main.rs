#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use kernel::memory::{Locked, FixedSizeBlockAllocator};
use task::{executor::Executor, Task};
use core::panic::PanicInfo;

use crate::{kernel::rendering::{Renderer, backend::cpu::CPURenderer}, loaders::image::tga::TGAImageFile};

extern crate alloc;

#[cfg(test)] pub mod tests;
#[rustfmt::skip] pub mod constants;

pub mod loaders;
pub mod kernel;
pub mod serial;
pub mod task;

#[global_allocator] pub static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    println_verbose!("Starting Hugo4OS...");
    
    println_verbose!("Kernel");
    println_verbose!("GDT");
    kernel::gdt::init();
    
    println_verbose!("Interrupts");
    kernel::interrupts::init();
    kernel::interrupts::disable();
    
    println_verbose!("Memory management");
    let physical_memory_offset = boot_info.physical_memory_offset.into_option().unwrap();
    kernel::memory::init(physical_memory_offset, &boot_info.memory_regions);
    
    println_verbose!("CPU-based renderer");
    
    let mut renderer = Renderer::new(boot_info.framebuffer.as_mut().unwrap(), CPURenderer::new());

    println_verbose!("Splash Screen");
        
    // Display splash screen
    
    let center_x = renderer.get_width() / 2;
    let center_y = renderer.get_height() / 2;
    
    let logo_top = center_y - 100;
    let logo_left = center_x - 100;
    let logo_right = logo_left + 200;
    let logo_bottom = logo_top + 200;

    let scale_texture = |tex: &[u32], width: usize, scale: usize| -> Vec<u32> {
        tex
            .to_vec()
            .chunks(width)
            .flat_map(|row| {
                let row_scaled = row
                    .into_iter()
                    // Repeat each pixel {scale} times
                    .flat_map(|pixel| (0..scale).map(|_|*pixel))
                    .collect::<Vec<_>>();

                (0..scale)
                    // Repeat each row {scale} times
                    .flat_map(move |_| row_scaled.clone())
            })
            .collect::<Vec<u32>>()
    };
    
    renderer.clear_screen();
    
    renderer.fill_rect(logo_left, logo_top, 200, 20, 0xffda0037);
    renderer.fill_rect(logo_left, logo_bottom - 20, 200, 20, 0xffda0037);

    renderer.fill_rect(logo_left + 50, center_y - 10, 100, 20, 0xffd3d3d3);
    renderer.fill_rect(logo_left + 50, logo_top + 40, 20, 120, 0xffd3d3d3);
    renderer.fill_rect(logo_right - 50 - 20, logo_top + 40, 20, 120, 0xffd3d3d3);

    renderer.blit_texture_blend(160, 0, 160, 20, scale_texture(constants::ALPHA_BLEND_TEST, 16, 10).as_slice());
    renderer.blit_texture_blend(128, 0, 32, 32, constants::PIXEL_ART);
    renderer.draw_char(0, 0, 'G', 128.0, 0xffd3d3d3);
    renderer.draw_string(0, 128, "Hugo4IT", 64.0, 0xffd3d3d3);

    renderer.fill_rect(0, 192, 64, 64, renderer.blend_colors(0x88171717, 0x88da0037));
    renderer.fill_rect(64, 192, 64, 64, 0xffda0037);

    let colors = [
        0x11171717,
        0x88da0037,
        renderer.blend_colors(0x11171717, 0x88da0037),
        renderer.get_pixel(64, 192)
    ];
    for (i, color) in colors.into_iter().enumerate() {
        let [r, g, b, a] = color.to_le_bytes();
        let r = u32::from_le_bytes([r, r, r, r]);
        let g = u32::from_le_bytes([g, g, g, g]);
        let b = u32::from_le_bytes([b, b, b, b]);
        let a = u32::from_le_bytes([a, a, a, a]);
        renderer.fill_rect(0, 256 + i * 16, 16, 16, r);
        renderer.fill_rect(16, 256 + i * 16, 16, 16, g);
        renderer.fill_rect(32, 256 + i * 16, 16, 16, b);
        renderer.fill_rect(48, 256 + i * 16, 16, 16, a);
    }

    renderer.present();

    let test_image = TGAImageFile::from_bytes(constants::TGA_TEST_IMAGE).unwrap();
    renderer.blit_image_blend(renderer.get_width()-400, renderer.get_height()-400, &test_image);
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