use bootloader::boot_info::{FrameBuffer, FrameBufferInfo};

use self::{backend::{RenderBackend, cpu::FrameBufferMakePublic}, font::FontRasterizer};

pub mod backend;
pub mod font;

pub struct Renderer<'a, B: RenderBackend> {
    backend: B,
    framebuffer: &'a mut FrameBuffer,
    buffer_info: FrameBufferInfo,

    font_rasterizer: FontRasterizer,
}

impl<'a, B: RenderBackend> Renderer<'a, B> {
    pub fn new(framebuffer: &mut FrameBuffer, mut backend: B) -> Renderer<B> {
        let buffer_info = framebuffer.info();
        backend.init(
            buffer_info.horizontal_resolution,
            buffer_info.vertical_resolution,
            buffer_info.bytes_per_pixel,
            buffer_info.stride,
            buffer_info.pixel_format
        );

        Renderer {
            backend,
            framebuffer,
            buffer_info,
            font_rasterizer: FontRasterizer::new(include_bytes!("../../../res/fonts/JetBrainsMono/JetBrains Mono Regular Nerd Font Complete Mono.ttf") as &[u8])
        }
    }

    pub fn draw_char(&mut self, x: usize, y: usize, ch: char, size: f32) {
        
    }

    #[inline]
    pub fn get_width(&self) -> usize {
        self.buffer_info.horizontal_resolution
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        self.buffer_info.vertical_resolution
    }

    #[inline]
    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        assert!(x + width < self.buffer_info.horizontal_resolution);
        assert!(y + height < self.buffer_info.vertical_resolution);
        unsafe { self.fill_rect_unchecked(x, y, width, height, color) }
    }

    #[inline]
    pub unsafe fn fill_rect_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        self.backend.fill_rect(x, y, width, height, color)
    }

    #[inline]
    pub fn blit_texture(&mut self, x: usize, y: usize, width: usize, height: usize, texture: *const u32) {
        assert!(x + width < self.buffer_info.horizontal_resolution);
        assert!(y + height < self.buffer_info.vertical_resolution);
        unsafe { self.blit_texture_unchecked(x, y, width, height, texture) }
    }

    #[inline]
    pub unsafe fn blit_texture_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, texture: *const u32) {
        self.backend.blit_texture(x, y, width, height, texture)
    }

    #[inline]
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        assert!(x <= self.buffer_info.horizontal_resolution);
        assert!(y <= self.buffer_info.vertical_resolution);
        unsafe { self.put_pixel_unchecked(x, y, color) }
    }

    #[inline]
    pub unsafe fn put_pixel_unchecked(&mut self, x: usize, y: usize, color: u32) {
        self.backend.put_pixel(x, y, color)
    }

    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        assert!(x <= self.buffer_info.horizontal_resolution);
        assert!(y <= self.buffer_info.vertical_resolution);
        unsafe { self.get_pixel_unchecked(x, y) }
    }

    #[inline]
    pub unsafe fn get_pixel_unchecked(&self, x: usize, y: usize) -> u32 {
        self.backend.get_pixel(x, y)
    }
    
    #[inline]
    pub fn set_clear_color(&mut self, color: u32) {
        self.backend.set_clear_color(color)
    }

    #[inline]
    pub fn clear_screen(&mut self) {
        self.backend.clear_screen()
    }

    /// Copy backend's backbuffer over to the framebuffer
    pub fn present(&mut self) {
        // Backbuffer
        let bb_start = self.backend.get_buffer();
        // Framebuffer
        let fb_start = unsafe { self.framebuffer.get_start_address() };

        let stride = self.buffer_info.stride;
        let height = self.buffer_info.vertical_resolution;
        let bpp = self.buffer_info.bytes_per_pixel;

        unsafe { core::ptr::copy(bb_start, fb_start, stride * height * bpp) };
    }
}