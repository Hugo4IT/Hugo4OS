use crate::{println, print, cpu_renderer, data};

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
fn check_test_framework() {
    assert_eq!(1, 1);
}

// Interrupts

#[test_case]
fn check_crash_catch() {
    x86_64::instructions::interrupts::int3();
}

// Rendering

#[test_case]
fn check_framebuffer_exists() {
    assert!(unsafe { cpu_renderer::FRAMEBUFFER.is_some() });
}

#[test_case]
fn check_set_background() {
    unsafe {
        cpu_renderer::clear_background();
        for y in 0..cpu_renderer::FB_HEIGHT {
            for x in 0..cpu_renderer::FB_WIDTH {
                assert!(cpu_renderer::get_pixel(x, y) == 0x17171717);
            }
        }
    }
}

#[test_case]
fn check_set_rect() {
    unsafe {
        cpu_renderer::set_rect(data::COLORS[data::RED], 10, 10, 100, 100);
        for y in 10..110 {
            for x in 10..110 {
                assert!(cpu_renderer::get_pixel(x, y) == data::COLORS[data::RED]);
            }
        }
    };
}