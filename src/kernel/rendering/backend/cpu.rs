use alloc::vec::Vec;
use bootloader::boot_info::{FrameBuffer, PixelFormat};

use crate::constants;
use crate::println_verbose;

use super::RenderBackend;

pub struct CPURenderer {
    buffer: Vec<u32>,
    format: PixelFormat,
    real_stride: usize,
    stride: usize,
    bpp: usize,
    clear_color: u32,
}

impl CPURenderer {
    pub fn new() -> CPURenderer {
        CPURenderer {
            buffer: Vec::new(),
            format: PixelFormat::BGR,
            real_stride: 0,
            stride: 0,
            bpp: 0,
            clear_color: 0xff171717,
        }
    }
}

impl RenderBackend for CPURenderer {
    fn init(
        &mut self,
        width: usize,
        height: usize,
        bytes_per_pixel: usize,
        stride: usize,
        format: PixelFormat,
    ) {
        self.buffer.resize(stride * height, 0);
        self.format = format;
        self.stride = stride;
        self.bpp = bytes_per_pixel;
        self.real_stride = self.bpp * self.stride;
    }

    unsafe fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let offset = x * self.bpp;
        let start = y * self.real_stride;
        let buffer = self.get_buffer_mut() as usize + start;

        for y in 0..height {
            for x in 0..width {
                core::ptr::write(
                    (buffer + offset + x * self.bpp + y * self.real_stride) as *mut u32,
                    color,
                )
            }
        }
    }

    unsafe fn blit_texture(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        texture: &[u32],
    ) {
        let offset = x * self.bpp;
        let start = y * self.real_stride;
        let buffer = self.get_buffer_mut() as usize + start;

        for y in 0..height {
            core::ptr::copy_nonoverlapping(
                &texture[y * width] as *const u32,
                (buffer + offset + y * self.real_stride) as *mut u32,
                width,
            )
        }
    }

    unsafe fn blit_texture_blend(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        texture: &[u32],
    ) {
        let offset = x * self.bpp;
        let start = y * self.real_stride;
        let buffer = self.get_buffer_mut() as usize + start;

        let lookup = (&constants::LERP_LOOKUP_TABLE[0] as *const u8) as usize;

        for y in 0..height {
            for x in 0..width {
                let index = offset + x * self.bpp + y * self.real_stride;
                let input_color = texture[y * width + x];
                let input_alpha = (input_color & 0xFF000000) >> 24;
                if input_alpha == 0 {
                    continue;
                } else if input_alpha == 255 {
                    core::ptr::write((buffer + index) as *mut u32, input_color);
                } else {
                    let dst = buffer + index;
                    // let o_px = core::ptr::read(dst as *const u32);

                    let input_red = (input_color & 0x00FF0000) >> 16;
                    let input_green = (input_color & 0x0000FF00) >> 8;
                    let input_blue = input_color & 0x000000FF;

                    // let value_r = constants::LERP_LOOKUP_TABLE[i_px_a * 255 + i_px_r];
                    let value_r = core::ptr::read((lookup + (input_alpha * 255 + input_red) as usize) as *const u8);
                    let value_g = core::ptr::read((lookup + (input_alpha * 255 + input_green) as usize) as *const u8);
                    let value_b = core::ptr::read((lookup + (input_alpha * 255 + input_blue) as usize) as *const u8);

                    let value = 0xFF000000 & ((value_r as u32) << 16) | ((value_g as u32) << 8) | (value_b as u32);

                    core::ptr::write(dst as *mut u32, value);
                }
            }
        }
    }

    fn set_clear_color(&mut self, color: u32) {
        self.clear_color = color;
    }

    fn clear_screen(&mut self) {
        let mut buffer = self.get_buffer_mut() as usize;
        let end = self.buffer.len() * self.bpp + buffer;
        while buffer <= end {
            unsafe { core::ptr::write(buffer as *mut u32, self.clear_color) };
            buffer += self.bpp;
        }
    }

    fn get_buffer(&self) -> *const u8 {
        self.buffer.as_ptr() as *const u8
    }

    fn get_buffer_mut(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr() as *mut u8
    }

    unsafe fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        let index = x * self.bpp + y * self.real_stride;
        core::ptr::write((self.get_buffer_mut() as usize + index) as *mut u32, color)
    }

    unsafe fn get_pixel(&self, x: usize, y: usize) -> u32 {
        let index = x * self.bpp + y * self.real_stride;
        core::ptr::read((self.get_buffer() as usize + index) as *const u32)
    }
}

pub trait FrameBufferMakePublic {
    unsafe fn get_start_address(&mut self) -> *mut u8;
}

impl FrameBufferMakePublic for FrameBuffer {
    unsafe fn get_start_address(&mut self) -> *mut u8 {
        self.buffer_mut().get_unchecked_mut(0) as *mut u8
    }
}