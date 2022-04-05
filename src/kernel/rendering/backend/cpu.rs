use alloc::vec::Vec;

use crate::{constants, kernel::abstractions::PixelFormat};

use super::RenderBackend;

macro_rules! color_expr {
    (mul $x:expr, $y:expr) => ((u16::from_le_bytes([constants::COLOR_MULT_LOOKUP_TABLE[(($x) * 256 + ($y)) as usize * 2], constants::COLOR_MULT_LOOKUP_TABLE[(($x) * 256 + ($y)) as usize * 2 + 1]]) as u32));
    (div $x:expr, $y:expr) => ((u16::from_le_bytes([constants::COLOR_DIV_LOOKUP_TABLE[(($x) * 256 + ($y)) as usize * 2], constants::COLOR_DIV_LOOKUP_TABLE[(($x) * 256 + ($y)) as usize * 2 + 1]]) as u32));
}

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
        _width: usize,
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

        for y in 0..height {
            for x in 0..width {
                let index = offset + x * self.bpp + y * self.real_stride;
                let dst = buffer + index;
                
                let texture_color = texture[y * width + x];
                let framebuffer_color = core::ptr::read(dst as *const u32);
                core::ptr::write(dst as *mut u32, self.overlay_color(framebuffer_color, texture_color));
            }
        }
    }

    fn overlay_color(&self, background: u32, foreground: u32) -> u32 {
        let [fg_b, fg_g, fg_r, fg_a] = foreground.to_le_bytes();
        let [bg_b, bg_g, bg_r, bg_a] = background.to_le_bytes();
            
        if fg_a == 0 {
            background
        } else if fg_a == 255 {
            foreground
        } else {
            let (fg_r, fg_g, fg_b, fg_a) = (fg_r as u32, fg_g as u32, fg_b as u32, fg_a as u32);
            let (bg_r, bg_g, bg_b, bg_a) = (bg_r as u32, bg_g as u32, bg_b as u32, bg_a as u32);

            let value_a = fg_a + color_expr!(mul bg_a, (255 - fg_a));
            let value_r = color_expr!(div color_expr!(mul fg_r, fg_a) + color_expr!(mul color_expr!(mul bg_r, bg_a), (255 - fg_a)), value_a);
            let value_g = color_expr!(div color_expr!(mul fg_g, fg_a) + color_expr!(mul color_expr!(mul bg_g, bg_a), (255 - fg_a)), value_a);
            let value_b = color_expr!(div color_expr!(mul fg_b, fg_a) + color_expr!(mul color_expr!(mul bg_b, bg_a), (255 - fg_a)), value_a);

            u32::from_le_bytes([value_b.min(255) as u8, value_g.min(255) as u8, value_r.min(255) as u8, value_a.min(255) as u8])
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

    unsafe fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let index = x * self.bpp + y * self.real_stride;
        core::ptr::write((self.get_buffer_mut() as usize + index) as *mut u32, color)
    }

    unsafe fn get_pixel(&self, x: usize, y: usize) -> u32 {
        let index = x * self.bpp + y * self.real_stride;
        core::ptr::read((self.get_buffer() as usize + index) as *const u32)
    }
}