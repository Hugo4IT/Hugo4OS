use bootloader::boot_info::{FrameBuffer, PixelFormat};

pub static mut FRAMEBUFFER: Option<&'static mut FrameBuffer> = None;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Rect(pub usize, pub usize, pub usize, pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Color(pub u8, pub u8, pub u8);

pub fn set_background(color: Color) {
    if let Some(framebuffer) = unsafe { FRAMEBUFFER.as_mut() } {
        let info = framebuffer.info();

        match info.pixel_format {
            PixelFormat::RGB => {
                for byte in framebuffer.buffer_mut().chunks_mut(info.bytes_per_pixel) {
                    *byte.get_mut(0).unwrap() = color.0;
                    *byte.get_mut(1).unwrap() = color.1;
                    *byte.get_mut(2).unwrap() = color.2;
                }
            },
            PixelFormat::BGR => {
                for byte in framebuffer.buffer_mut().chunks_mut(info.bytes_per_pixel) {
                    *byte.get_mut(0).unwrap() = color.2;
                    *byte.get_mut(1).unwrap() = color.1;
                    *byte.get_mut(2).unwrap() = color.0;
                }
            },
            PixelFormat::U8 => {
                for byte in framebuffer.buffer_mut().chunks_mut(info.bytes_per_pixel) {
                    *byte.get_mut(0).unwrap() = (((color.0 as i32) + (color.1 as i32) + (color.2 as i32)) / 3) as u8;
                }
            },
            _ => ()
        }
    }
}

pub fn set_rect(rect: Rect, color: Color) {
    if let Some(framebuffer) = unsafe { FRAMEBUFFER.as_mut() } {
        let info = framebuffer.info();

        let mut rows = framebuffer.buffer_mut().chunks_mut(info.stride * info.bytes_per_pixel);
        for _i in 0..rect.1 { rows.next().unwrap(); } // Set y
        
        match info.pixel_format {
            PixelFormat::RGB => {
                for _i in 0..rect.3 {
                    let mut row = rows.next().unwrap().chunks_mut(info.bytes_per_pixel);
                    for _j in 0..rect.0 { row.next().unwrap(); } // Set x
                    for _j in 0..rect.2 {
                        let byte = row.next().unwrap();
                        *byte.get_mut(0).unwrap() = color.0;
                        *byte.get_mut(1).unwrap() = color.1;
                        *byte.get_mut(2).unwrap() = color.2;
                    }
                }
            },
            PixelFormat::BGR => {
                for _i in 0..rect.3 {
                    let mut row = rows.next().unwrap().chunks_mut(info.bytes_per_pixel);
                    for _j in 0..rect.0 { row.next().unwrap(); } // Set x
                    for _j in 0..rect.2 {
                        let byte = row.next().unwrap();
                        *byte.get_mut(0).unwrap() = color.2;
                        *byte.get_mut(1).unwrap() = color.1;
                        *byte.get_mut(2).unwrap() = color.0;
                    }
                }
            },
            PixelFormat::U8 => {
                for _i in 0..rect.3 {
                    let mut row = rows.next().unwrap().chunks_mut(info.bytes_per_pixel);
                    for _j in 0..rect.0 { row.next().unwrap(); } // Set x
                    for _j in 0..rect.2 {
                        let byte = row.next().unwrap();
                        *byte.get_mut(0).unwrap() = (((color.0 as i32) + (color.1 as i32) + (color.2 as i32)) / 3) as u8;
                    }
                }
            },
            _ => ()
        }
    }
}