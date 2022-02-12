#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

pub mod global_desc_table;
pub mod cpu_renderer;
pub mod interrupts;
pub mod serial;
pub mod tests;
pub mod data;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    global_desc_table::init();
    interrupts::init();

    unsafe {
        interrupts::with_disabled(||{
            cpu_renderer::set_framebuffer(boot_info.framebuffer.as_mut().unwrap());
    
            #[cfg(test)]
            test_main();
    
            cpu_renderer::blit_image(data::PIXEL_ART, 0, 0, 32, 32);
            cpu_renderer::set_rect(data::COLORS[data::RED], 48, 48, 64, 64);
            cpu_renderer::blit_image(data::PIXEL_ART, 64, 64, 32, 32);
        });

        // let mut frame: usize = 0;
        // let h_center = (cpu_renderer::FB_WIDTH / 2) as isize;
        // let v_center = (cpu_renderer::FB_HEIGHT / 2) as isize;
        // let mut x: f32 = 0.0;
        // loop {
        //     cpu_renderer::clear_background();
        //     for i in 0..100 {
        //         cpu_renderer::blit_image(
        //             data::PIXEL_ART,
        //             (h_center + (libm::sinf(x.to_radians() + i as f32) * 200.0) as isize) as usize,
        //             (v_center + (libm::cosf(x.to_radians() + i as f32) * 200.0) as isize) as usize,
        //             32,
        //             32
        //         );
        //     }
        //     x += 1.0;
        //     frame += 1;
        //     println!("Frame: {}", frame);
        // }
    }

    loop {}
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    println!("Failed!");
    println!("Error: {}", info);
    tests::exit_qemu(0x10);
    loop {}
}
