use bootloader::boot_info::{FrameBuffer, PixelFormat};

use crate::println;
use crate::data;

pub static mut FRAMEBUFFER: Option<&'static mut FrameBuffer> = None;
pub static mut FB_BPP: usize = 0;
pub static mut FB_ACTUAL_STRIDE: usize = 0;
pub static mut FB_FORMAT: PixelFormat = PixelFormat::RGB;
pub static mut FB_START: *mut u8 = 0u8 as *mut u8;
pub static mut FB_WIDTH: usize = 0;
pub static mut FB_HEIGHT: usize = 0;

pub unsafe fn set_framebuffer(fb: &'static mut FrameBuffer) {
    let info = fb.info();
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
    
    FB_BPP = info.bytes_per_pixel;
    FB_ACTUAL_STRIDE = FB_BPP * info.stride;
    FB_FORMAT = info.pixel_format;
    FB_WIDTH = info.horizontal_resolution;
    FB_HEIGHT = info.vertical_resolution;
    FB_START = fb.get_start_address();
    FRAMEBUFFER = Some(fb);

    clear_background();

    println!("Loading images...");
    ensure_format(data::PIXEL_ART, PixelFormat::RGB);
    println!("Done loading...");
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
    core::ptr::write_bytes(FB_START, 0x17, FB_ACTUAL_STRIDE * FRAMEBUFFER.as_ref().unwrap().info().vertical_resolution)
}

pub unsafe fn set_rect(color: u32, x: usize, y: usize, width: usize, height: usize) {
    let framebuffer = FRAMEBUFFER.as_mut().unwrap();

    for y in y..(y+height) {
        for x in x..(x+width) {
            ((FB_START as usize + y * FB_ACTUAL_STRIDE + x * FB_BPP) as *mut u32).write(color);
        }
    }
}

pub unsafe fn blit_image(data: &[u32], x: usize, y: usize, width: usize, height: usize) {
    let framebuffer = FRAMEBUFFER.as_mut().unwrap();

    for (i, y) in (y..(y+height)).enumerate() {
        core::ptr::copy_nonoverlapping(&data[i * width] as *const u32, (FB_START as usize + y * FB_ACTUAL_STRIDE + x * FB_BPP) as *mut u32, width);
    }
}