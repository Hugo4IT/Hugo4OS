use bootloader::boot_info::{FrameBuffer, PixelFormat};

use crate::constants;
use crate::println_verbose;

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

    #[rustfmt::skip]
    println_verbose!(concat!(   "===================================== \n",
                                "  Display information                 \n",
                                "===================================== \n",
                                "  Resolution          | {}x{}         \n",
                                "  Pixel format        | {}            \n",
                                "  Bytes per pixel     | {}            \n",
                                "===================================== \n"),
                                width, height, _pixel_format, _bytes_per_pixel);
    
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