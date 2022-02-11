use crate::{println, print, cpu_renderer::{self, Color, Rect}};

pub fn exit_qemu(exit_code: u32) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for (_i, test) in tests.iter().enumerate() {
        test.run();
    }

    println!("\nThis is a false positive:");
    exit_qemu(0x11);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("[{}] Running... ", core::any::type_name::<T>());
        self();
        println!("Ok!");
    }
}

#[test_case]
fn test_test_framework() {
    assert_eq!(1, 1);
}

#[test_case]
fn check_framebuffer_exists() {
    assert!(unsafe { cpu_renderer::FRAMEBUFFER.is_some() });
}

#[test_case]
fn check_set_background() {
    cpu_renderer::set_background(Color(0xde, 0x91, 0x51));
    let framebuffer = unsafe { cpu_renderer::FRAMEBUFFER.as_ref().unwrap() };
    match framebuffer.info().pixel_format {
        bootloader::boot_info::PixelFormat::RGB => {
            for byte in framebuffer.buffer().chunks(framebuffer.info().bytes_per_pixel) {
                assert!(byte[0] == 0xde);
                assert!(byte[1] == 0x91);
                assert!(byte[2] == 0x51);
            }
        },
        bootloader::boot_info::PixelFormat::BGR => {
            for byte in framebuffer.buffer().chunks(framebuffer.info().bytes_per_pixel) {
                assert!(byte[0] == 0x51);
                assert!(byte[1] == 0x91);
                assert!(byte[2] == 0xde);
            }
        },
        bootloader::boot_info::PixelFormat::U8 => {
            for byte in framebuffer.buffer().chunks(framebuffer.info().bytes_per_pixel) {
                assert!(byte[0] == 0x95);
                assert!(byte[1] == 0x95);
                assert!(byte[2] == 0x95);
            }
        },
        _ => unreachable!()
    }
}

#[test_case]
fn check_set_rect() {
    cpu_renderer::set_rect(Rect(10, 10, 100, 100), Color(0xf3, 0x42, 0x13));
    let framebuffer = unsafe { cpu_renderer::FRAMEBUFFER.as_ref().unwrap() };
    match framebuffer.info().pixel_format {
        bootloader::boot_info::PixelFormat::RGB => {
            for row in 10..110 {
                for col in 10..110 {
                    let pos = row * framebuffer.info().stride + col;
                    for byte in framebuffer.buffer().get((pos * framebuffer.info().bytes_per_pixel)..((pos+4) * framebuffer.info().bytes_per_pixel)) {
                        assert!(byte[0] == 0xf3);
                        assert!(byte[1] == 0x42);
                        assert!(byte[2] == 0x13);
                    }
                }
            }
        },
        bootloader::boot_info::PixelFormat::BGR => {
            for row in 10..110 {
                for col in 10..110 {
                    let pos = row * framebuffer.info().stride + col;
                    for byte in framebuffer.buffer().get((pos * framebuffer.info().bytes_per_pixel)..((pos+4) * framebuffer.info().bytes_per_pixel)) {
                        assert!(byte[0] == 0x13);
                        assert!(byte[1] == 0x42);
                        assert!(byte[2] == 0xf3);
                    }
                }
            }
        },
        bootloader::boot_info::PixelFormat::U8 => {
            for row in 10..110 {
                for col in 10..110 {
                    let pos = row * framebuffer.info().stride + col;
                    for byte in framebuffer.buffer().get((pos * framebuffer.info().bytes_per_pixel)..((pos+4) * framebuffer.info().bytes_per_pixel)) {
                        assert!(byte[0] == 0x6d);
                        assert!(byte[1] == 0x6d);
                        assert!(byte[2] == 0x6d);
                    }
                }
            }
        },
        _ => unreachable!()
    }
}