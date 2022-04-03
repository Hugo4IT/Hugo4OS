use bootloader::boot_info::PixelFormat;

pub mod cpu;

pub trait RenderBackend {
    fn init(&mut self, width: usize, height: usize, bytes_per_pixel: usize, stride: usize, format: PixelFormat);
    unsafe fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32);
    unsafe fn blit_texture(&mut self, x: usize, y: usize, width: usize, height: usize, texture: &[u32]);
    unsafe fn blit_texture_blend(&mut self, x: usize, y: usize, width: usize, height: usize, texture: &[u32]);
    unsafe fn set_pixel(&mut self, x: usize, y: usize, color: u32);
    unsafe fn get_pixel(&self, x: usize, y: usize) -> u32;
    fn set_clear_color(&mut self, color: u32);
    fn clear_screen(&mut self);

    fn overlay_color(&self, background: u32, foreground: u32) -> u32;

    fn get_buffer(&self) -> *const u8;
    fn get_buffer_mut(&mut self) -> *mut u8;
}