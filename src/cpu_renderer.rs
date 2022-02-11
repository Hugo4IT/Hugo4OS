use bootloader::boot_info::{FrameBuffer, PixelFormat};

use crate::println;

pub static mut FRAMEBUFFER: Option<&'static mut FrameBuffer> = None;

pub trait FrameBufferMakePublic {
    unsafe fn get_start_address(&mut self) -> *mut u8;
}

impl FrameBufferMakePublic for FrameBuffer {
    unsafe fn get_start_address(&mut self) -> *mut u8 {
        self.buffer_mut().get_unchecked_mut(0) as *mut u8
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Rect(pub usize, pub usize, pub usize, pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn from_argb(argb: u32) -> Color {
        Color(
            ((argb & 0x00FF0000) >> 16) as u8,
            ((argb & 0x0000FF00) >> 8) as u8,
            (argb & 0x000000FF) as u8,
        )
    }

    pub fn to_pixel(self, format: PixelFormat) -> [u8; 3] {
        match format {
            PixelFormat::RGB => [self.0, self.1, self.2],
            PixelFormat::BGR => [self.2, self.1, self.0],
            PixelFormat::U8 => {
                let luminance = (((self.0 + self.1 + self.2) as f32) / 3.0) as u8;
                [luminance, luminance, luminance]
            },
            _ => [0, 0, 0],
        }
    }
}

pub fn set_background(color: Color) {
    if let Some(framebuffer) = unsafe { FRAMEBUFFER.as_mut() } {
        let info = framebuffer.info();
        set_rect(Rect(0, 0, info.horizontal_resolution, info.vertical_resolution), color);
    }
}

pub fn set_rect(rect: Rect, color: Color) {
    if let Some(framebuffer) = unsafe { FRAMEBUFFER.as_mut() } {
        let info = framebuffer.info();
        let start_addr = unsafe { framebuffer.get_start_address() };
        let color = color.to_pixel(info.pixel_format);
        
        for y in rect.1..(rect.1+rect.3) {
            for x in rect.0..(rect.0+rect.2) {
                unsafe {
                    *((start_addr as usize + (y * info.bytes_per_pixel * info.stride + x * info.bytes_per_pixel)) as *mut [u8; 3]) = color;
                }
            }
        }
    }
}

pub fn blit_art(x: usize, y: usize, data: &[u32], width: usize, height: usize) {
    if let Some(framebuffer) = unsafe { FRAMEBUFFER.as_mut() } {
        let info = framebuffer.info();
        let start_addr = unsafe { framebuffer.get_start_address() };

        let rows = unsafe{ data.as_chunks(width).map(|(c, _)|c as *[u32; width]).collect::<Vec<*[u32; width]>>() };

        for y in y..(y+height) {
    //         for x in x..(x+width) {
    //             unsafe {
    //                 *((start_addr as usize + (y * info.horizontal_resolution * info.stride + x * info.stride)) as *mut u32) = 0u32;
    //             }
    //         }
    //         // buffer[(i * info.horizontal_resolution + x)..(i * info.horizontal_resolution + x + width)] = data[((i - y) * width)..((i - y) * width + width)];
        }
    }
}