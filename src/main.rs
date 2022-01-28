#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{boot_info::PixelFormat, entry_point, BootInfo};
use core::panic::PanicInfo;

use crate::cpu_renderer::{Color, Rect};

pub mod cpu_renderer;
pub mod serial;
pub mod tests;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    unsafe {
        cpu_renderer::FRAMEBUFFER.insert(boot_info.framebuffer.as_mut().unwrap());
        let info = cpu_renderer::FRAMEBUFFER.as_ref().unwrap().info();

        let pixel_format = match info.pixel_format {
            PixelFormat::RGB => "RGB",
            PixelFormat::BGR => "BGR",
            PixelFormat::U8 => "U8 (Grayscale)",
            other => panic!("Unrecognized pixel format: {:?}", other),
        };
        let bytes_per_pixel = info.bytes_per_pixel;
        let width = info.horizontal_resolution;
        let height = info.vertical_resolution;

        #[rustfmt::skip]
        println!(concat!(   "===================================== \n",
                            "  Display information                 \n",
                            "===================================== \n",
                            "  Resolution          | {}x{}         \n",
                            "  Pixel format        | {}            \n",
                            "  Bytes per pixel     | {}            \n",
                            "===================================== \n"),
                            width, height, pixel_format, bytes_per_pixel);
    }

    #[cfg(test)]
    test_main();

    cpu_renderer::set_background(Color(0x17, 0x17, 0x17));
    cpu_renderer::set_rect(Rect(150, 200, 300, 150), Color(0xda, 0x00, 0x37));

    println!("Goodbye, {}!", "World");

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
