pub mod rendering {
    #[derive(Debug, Clone, Copy)]
    pub enum PixelFormat {
        RGB,
        BGR,
        U8
    }
    
    #[derive(Debug, Clone, Copy)]
    pub struct FrameBufferInfo {
        pub width: usize,
        pub height: usize,
        pub bytes_per_pixel: usize,
        pub stride: usize,
        pub pixel_format: PixelFormat,
    }
    
    pub trait FrameBuffer {
        fn info(&self) -> FrameBufferInfo;
        fn get_start_address(&self) -> *mut u8;
    }
}

pub mod interrupts {
    #[derive(Debug, Clone, Copy)]
    pub struct InputSyscall {
        pub target: *mut u64, // *mut, as it also functions as the return value
        pub args: [u64; 11],
    }
}