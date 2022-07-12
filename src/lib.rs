#![no_std]
#![no_main]

extern crate alloc;

use fontdue::{Font, FontSettings};

use kernel::{rendering::{Renderer, backend::cpu::CPURenderer}, architecture::Architecture, interrupts::Interrupts};
use task::{executor::Executor, Task};

#[cfg(test)] pub mod tests;
#[rustfmt::skip] pub mod constants;

pub mod loaders;
pub mod kernel;
pub mod task;
pub mod util;

// TODO: Add aarch64 support
// TODO: Make `kernel_main` architecture independent, with bootloaders starting it instead of the other way around

pub fn kernel_main<Arch: Architecture>(framebuffer: Arch::FrameBuffer) -> ! {
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
    renderer.fonts.push(Font::from_bytes(constants::FONT_NERD_MONO, FontSettings::default()).unwrap());

    renderer.clear_screen();
    renderer.present();

    <Arch as Interrupts>::enable();

    for i in 0..120 {
        renderer.clear_screen();
        renderer.fill_rect(i * 5, i * 2, 32, 32, 0xffd3d3d3);
        renderer.present();
    }

    let mut executor = Executor::new();
    executor.spawn(Task::new(task::keyboard::print_keypresses()));
    executor.run::<Arch>();
}