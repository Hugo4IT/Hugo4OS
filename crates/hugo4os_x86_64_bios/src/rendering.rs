use bootloader::boot_info;

use hugo4os::kernel::abstractions::rendering::{self as abstractions, PixelFormat};

pub struct FrameBuffer {
    info: abstractions::FrameBufferInfo,
    start_address: u64,
}

impl FrameBuffer {
    pub(crate) fn new(framebuffer: &boot_info::FrameBuffer) -> Self {
        let info = framebuffer.info();

        Self {
            info: abstractions::FrameBufferInfo {
                width: info.horizontal_resolution,
                height: info.vertical_resolution,
                bytes_per_pixel: info.bytes_per_pixel,
                stride: info.stride,
                pixel_format: match info.pixel_format {
                    boot_info::PixelFormat::BGR => abstractions::PixelFormat::BGR,
                    boot_info::PixelFormat::RGB => abstractions::PixelFormat::RGB,
                    boot_info::PixelFormat::U8 => abstractions::PixelFormat::U8,
                    _ => unreachable!()
                },
            },
            start_address: framebuffer.buffer_start,
        }
    }
}

impl abstractions::FrameBuffer for FrameBuffer {
    #[inline]
    fn info(&self) -> abstractions::FrameBufferInfo {
        self.info
    }
    
    #[inline]
    fn get_start_address(&self) -> *mut u8 {
        self.start_address as *mut u8
    }
}