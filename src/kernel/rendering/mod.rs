//! Generic renderer with a target framebuffer and a backend.

pub mod backend;

use alloc::vec::Vec;
use fontdue::{Font, layout::{Layout, CoordinateSystem, TextStyle}};

use crate::{loaders::image::Image, kernel::abstractions::rendering::{FrameBuffer, FrameBufferInfo}};
use backend::RenderBackend;

pub struct Renderer<F: FrameBuffer, B: RenderBackend> {
    backend: B,
    framebuffer: F,
    buffer_info: FrameBufferInfo,
    pub fonts: Vec<Font>,
}

impl<F: FrameBuffer, B: RenderBackend> Renderer<F, B> {
    pub fn new(framebuffer: F, mut backend: B) -> Renderer<F, B> {
        let buffer_info = framebuffer.info();
        backend.init(
            buffer_info.width,
            buffer_info.height,
            buffer_info.bytes_per_pixel,
            buffer_info.stride,
            buffer_info.pixel_format
        );

        Renderer {
            backend,
            framebuffer,
            buffer_info,
            fonts: Vec::new(),
        }
    }

    fn convert_glyph_texture(&self, texture: Vec<u8>, color: u32) -> Vec<u32> {
        texture.into_iter().map(|l| ((l as u32)<<24)|(color&0x00FFFFFF)).collect::<Vec<u32>>()
    }

    pub fn draw_char(&mut self, x: usize, y: usize, ch: char, size: f32, color: u32) {
        let (metrics, texture) = self.fonts[0].rasterize(ch, size);
        self.blit_texture_blend(x, y, metrics.width, metrics.height, self.convert_glyph_texture(texture, color).as_slice())
    }

    pub unsafe fn draw_char_unchecked(&mut self, x: usize, y: usize, ch: char, size: f32, color: u32) {
        let (metrics, texture) = self.fonts[0].rasterize(ch, size);
        self.blit_texture_blend_unchecked(x, y, metrics.width, metrics.height, self.convert_glyph_texture(texture, color).as_slice())
    }

    pub fn draw_string(&mut self, x: usize, y: usize, string: &str, size: f32, color: u32) {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.append(self.fonts.as_slice(), &TextStyle::new(string, size, 0));

        for glyph in layout.glyphs() {
            let (metrics, texture) = self.fonts[glyph.font_index].rasterize_config(glyph.key);
            self.blit_texture_blend(
                x + glyph.x as usize,
                y + glyph.y as usize,
                metrics.width,
                metrics.height,
                self.convert_glyph_texture(texture, color).as_slice()
            )
        }
    }

    #[inline]
    pub fn get_width(&self) -> usize {
        self.buffer_info.width
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        self.buffer_info.height
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        assert!(x + width <= self.buffer_info.width);
        assert!(y + height <= self.buffer_info.height);
        unsafe { self.fill_rect_unchecked(x, y, width, height, color) }
    }

    #[inline]
    pub unsafe fn fill_rect_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        self.backend.fill_rect(x, y, width, height, color)
    }

    #[inline]
    pub unsafe fn blit_image<I: Image>(&mut self, x: usize, y: usize, image: &I) {
        self.blit_texture(x, y, image.get_width(), image.get_height(), image.get_texture().as_slice())
    }

    #[inline]
    pub unsafe fn blit_image_unchecked<I: Image>(&mut self, x: usize, y: usize, image: &I) {
        self.blit_texture_unchecked(x, y, image.get_width(), image.get_height(), image.get_texture().as_slice())
    }

    #[inline]
    pub fn blit_image_blend<I: Image>(&mut self, x: usize, y: usize, image: &I) {
        self.blit_texture_blend(x, y, image.get_width(), image.get_height(), image.get_texture().as_slice())
    }

    #[inline]
    pub unsafe fn blit_image_blend_unchecked<I: Image>(&mut self, x: usize, y: usize, image: &I) {
        self.blit_texture_blend_unchecked(x, y, image.get_width(), image.get_height(), image.get_texture().as_slice())
    }

    pub fn blit_texture(&mut self, x: usize, y: usize, width: usize, height: usize, texture: &[u32]) {
        assert!(x + width <= self.buffer_info.width);
        assert!(y + height <= self.buffer_info.height);
        unsafe { self.blit_texture_unchecked(x, y, width, height, texture) }
    }

    #[inline]
    pub unsafe fn blit_texture_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, texture: &[u32]) {
        self.backend.blit_texture(x, y, width, height, texture)
    }

    pub fn blit_texture_blend(&mut self, x: usize, y: usize, width: usize, height: usize, texture: &[u32]) {
        assert!(x + width <= self.buffer_info.width);
        assert!(y + height <= self.buffer_info.height);
        unsafe { self.blit_texture_blend_unchecked(x, y, width, height, texture) }
    }

    #[inline]
    pub unsafe fn blit_texture_blend_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, texture: &[u32]) {
        self.backend.blit_texture_blend(x, y, width, height, texture)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        assert!(x <= self.buffer_info.width);
        assert!(y <= self.buffer_info.height);
        unsafe { self.set_pixel_unchecked(x, y, color) }
    }

    #[inline]
    pub unsafe fn set_pixel_unchecked(&mut self, x: usize, y: usize, color: u32) {
        self.backend.set_pixel(x, y, color)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        assert!(x <= self.buffer_info.width);
        assert!(y <= self.buffer_info.height);
        unsafe { self.get_pixel_unchecked(x, y) }
    }

    #[inline]
    pub unsafe fn get_pixel_unchecked(&self, x: usize, y: usize) -> u32 {
        self.backend.get_pixel(x, y)
    }
    
    #[inline]
    pub fn blend_colors(&self, x: u32, y: u32) -> u32 {
        self.backend.overlay_color(x, y)
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
        let fb_start = self.framebuffer.get_start_address();

        let stride = self.buffer_info.stride;
        let height = self.buffer_info.height;
        let bpp = self.buffer_info.bytes_per_pixel;

        unsafe { core::ptr::copy_nonoverlapping(bb_start, fb_start, stride * height * bpp) };
    }
}