use alloc::vec::Vec;
use bootloader::boot_info::{FrameBuffer, PixelFormat};

use crate::constants;
use crate::println_verbose;

use backend::RenderBackend;

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
    fn init(&mut self, width: usize, height: usize, bytes_per_pixel: usize, stride: usize, format: PixelFormat) {
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
                core::ptr::write((buffer + offset + x * self.bpp + y * self.real_stride) as *mut u32, color)
            }
        }
    }

    unsafe fn blit_texture(&mut self, x: usize, y: usize, width: usize, height: usize, texture: *const u32) {
        let offset = x * self.bpp;
        let start = y * self.real_stride;
        let buffer = self.get_buffer_mut() as usize + start;

        for y in 0..height {
            core::ptr::copy_nonoverlapping(
                (texture as usize + y * width) as *const u32,
                (buffer + offset + y * self.real_stride) as *mut u32,
                width,
            )
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

pub static mut FRAMEBUFFER: Option<&'static mut FrameBuffer> = None;
pub static mut FB_BPP: usize = 0;
pub static mut FB_ACTUAL_STRIDE: usize = 0;
pub static mut FB_FORMAT: PixelFormat = PixelFormat::RGB;
pub static mut FB_START: *mut u8 = 0u8 as *mut u8;
pub static mut FB_WIDTH: usize = 0;
pub static mut FB_HEIGHT: usize = 0;

// pub static mut BACKBUFFER: &mut [u8] = &mut [];
// pub static mut BB_START: *mut u8 = 0u8 as *mut u8;

pub unsafe fn init(fb: &'static mut FrameBuffer) {
    let info = fb.info();
    let _pixel_format = match info.pixel_format {
        PixelFormat::RGB => "RGB",
        PixelFormat::BGR => "BGR",
        PixelFormat::U8 => "U8 (Grayscale)",
        other => panic!("Unrecognized pixel format: {:?}", other),
    };
    let _bytes_per_pixel = info.bytes_per_pixel;
    let width = info.horizontal_resolution;
    let height = info.vertical_resolution;

    println_verbose!("=====================================");
    println_verbose!("  Display information                ");
    println_verbose!("=====================================");
    println_verbose!("  Resolution          | {}x{}        ", width, height);
    println_verbose!("  Pixel format        | {}           ", _pixel_format);
    println_verbose!("  Bytes per pixel     | {}           ", _bytes_per_pixel);
    println_verbose!("=====================================");
    
    FB_BPP = info.bytes_per_pixel;
    FB_ACTUAL_STRIDE = FB_BPP * info.stride;
    FB_FORMAT = info.pixel_format;
    FB_WIDTH = info.horizontal_resolution;
    FB_HEIGHT = info.vertical_resolution;
    FB_START = fb.get_start_address();
    FRAMEBUFFER = Some(fb);

    clear_background();

    // Draw splash screen

    let center_x = width / 2;
    let center_y = height / 2;

    let logo_top = center_y - 100;
    let logo_left = center_x - 100;
    let logo_right = logo_left + 200;
    let logo_bottom = logo_top + 200;


    fill_rect(constants::COLORS[constants::RED], logo_left, logo_top, 200, 20);
    fill_rect(constants::COLORS[constants::RED], logo_left, logo_bottom - 20, 200, 20);

    fill_rect(constants::COLORS[constants::FG], logo_left + 50, center_y - 10, 100, 20);
    fill_rect(constants::COLORS[constants::FG], logo_left + 50, logo_top + 40, 20, 120);
    fill_rect(constants::COLORS[constants::FG], logo_right - 50 - 20, logo_top + 40, 20, 120);

    println_verbose!("Loading images...");
    ensure_format(constants::PIXEL_ART, PixelFormat::RGB);
    println_verbose!("Done loading...");
}

pub trait FrameBufferMakePublic {
    unsafe fn get_start_address(&mut self) -> *mut u8;
}

impl FrameBufferMakePublic for FrameBuffer {
    unsafe fn get_start_address(&mut self) -> *mut u8 {
        self.buffer_mut().get_unchecked_mut(0) as *mut u8
    }
}

#[inline]
pub fn is_same_format(left: PixelFormat, right: PixelFormat) -> bool {
    match (left, right) {
        (PixelFormat::RGB, PixelFormat::RGB) => true,
        (PixelFormat::BGR, PixelFormat::BGR) => true,
        (PixelFormat::U8, PixelFormat::U8) => true,
        _ => false,
    }
}

pub fn convert_color(from: u32, from_format: PixelFormat, to: PixelFormat) -> u32 {
    if is_same_format(from_format, to) {
        return from;
    }

    let argb = match from_format {
        PixelFormat::RGB => {
            from
        },
        PixelFormat::BGR => {
            let abgr = from.to_be_bytes();
            u32::from_be_bytes([
                abgr[0],
                abgr[3],
                abgr[2],
                abgr[1],
            ])
        },
        _ => return from
    };

    match to {
        PixelFormat::RGB => {
            argb
        },
        PixelFormat::BGR => {
            let argb = argb.to_be_bytes();
            u32::from_be_bytes([
                argb[0],
                argb[3],
                argb[2],
                argb[1],
            ])
        },
        _ => return from
    }
}

pub unsafe fn ensure_format(data: &mut [u32], format: PixelFormat) {
    if !is_same_format(format, FB_FORMAT) {
        for color in data.iter_mut() {
            *color = convert_color(*color, format, FB_FORMAT);
        }
    }
}

pub unsafe fn clear_background() {
    core::ptr::write_bytes(FB_START, 0x17, FB_ACTUAL_STRIDE * FB_HEIGHT)
}

pub unsafe fn fill_rect(color: u32, x: usize, y: usize, width: usize, height: usize) {
    for y in y..(y+height) {
        for x in x..(x+width) {
            get_pointer(x, y).write_volatile(color);
        }
    }
}

pub unsafe fn blit_image(data: &[u32], x: usize, y: usize, width: usize, height: usize) {
    for (i, y) in (y..(y+height)).enumerate() {
        core::ptr::copy_nonoverlapping(&data[i * width] as *const u32, get_pointer(x, y), width);
    }
}

#[inline(always)]
pub unsafe fn get_pointer(x: usize, y: usize) -> *mut u32 {
    (FB_START as usize + y * FB_ACTUAL_STRIDE + x * FB_BPP) as *mut u32
}

#[inline]
pub unsafe fn get_pixel(x: usize, y: usize) -> u32 {
    *get_pointer(x, y)
}