use bootloader::boot_info;

use crate::kernel::abstractions;

pub trait FrameBufferMakePublic {
    unsafe fn get_start_address(&self) -> *const u8;
}

impl FrameBufferMakePublic for boot_info::FrameBuffer {
    #[inline]
    unsafe fn get_start_address(&self) -> *const u8 {
        self.buffer().get_unchecked(0) as *const u8
    }
}

impl abstractions::FrameBuffer for boot_info::FrameBuffer {
    #[inline]
    fn info(&self) -> abstractions::FrameBufferInfo {
        self.info().into()
    }
    
    #[inline]
    fn get_start_address(&self) -> *mut u8 {
        unsafe { <Self as FrameBufferMakePublic>::get_start_address(&self) as *mut u8 }
    }
}

impl Into<abstractions::PixelFormat> for boot_info::PixelFormat {
    fn into(self) -> abstractions::PixelFormat {
        match self {
            boot_info::PixelFormat::BGR => abstractions::PixelFormat::BGR,
            boot_info::PixelFormat::RGB => abstractions::PixelFormat::RGB,
            boot_info::PixelFormat::U8 => abstractions::PixelFormat::U8,
            _ => unreachable!()
        }
    }
}

impl Into<abstractions::FrameBufferInfo> for boot_info::FrameBufferInfo {
    fn into(self) -> abstractions::FrameBufferInfo {
        abstractions::FrameBufferInfo {
            width: self.horizontal_resolution,
            height: self.vertical_resolution,
            bytes_per_pixel: self.bytes_per_pixel,
            stride: self.stride,
            pixel_format: self.pixel_format.into(),
        }
    }
}