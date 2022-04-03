use ab_glyph::{FontRef, Font, OutlinedGlyph};
use bootloader::boot_info::{FrameBuffer, FrameBufferInfo};

use crate::{println_verbose, constants};

use self::backend::{RenderBackend, cpu::FrameBufferMakePublic};

pub mod backend;

pub struct Renderer<'a, B: RenderBackend> {
    backend: B,
    framebuffer: &'a mut FrameBuffer,
    buffer_info: FrameBufferInfo,
    font: FontRef<'a>,
}

impl<'a, B: RenderBackend> Renderer<'a, B> {
    pub fn new(framebuffer: &mut FrameBuffer, mut backend: B) -> Renderer<B> {
        println_verbose!("  Backend");

        let buffer_info = framebuffer.info();
        backend.init(
            buffer_info.horizontal_resolution,
            buffer_info.vertical_resolution,
            buffer_info.bytes_per_pixel,
            buffer_info.stride,
            buffer_info.pixel_format
        );

        println_verbose!("  Font");

        let font = FontRef::try_from_slice(constants::FONT_REGULAR).unwrap();
        
        println_verbose!("done");

        Renderer {
            backend,
            framebuffer,
            buffer_info,
            font,
        }
    }

    unsafe fn blit_char(&mut self, x: usize, y: usize, glyph: OutlinedGlyph, color: u32) {
        glyph.draw(|px, py, l| {
            let l = (l * 255.0) as u32;
            self.put_pixel_unchecked(px as usize + x, py as usize + y, ((l << 24) & 0xFF000000) | (color & 0x00FFFFFF))
        });
    }

    pub fn draw_char(&mut self, x: usize, y: usize, ch: char, size: f32, color: u32) {
        let glyph = self.font.glyph_id(ch).with_scale(size);
        let glyph = self.font.outline_glyph(glyph).unwrap();

        assert!(glyph.px_bounds().min.x >= 0 as f32);
        assert!(glyph.px_bounds().min.y >= 0 as f32);
        assert!(glyph.px_bounds().max.x < self.get_width() as f32);
        assert!(glyph.px_bounds().max.y < self.get_height() as f32);
        
        unsafe { self.blit_char(x, y, glyph, color) }
    }

    pub unsafe fn draw_char_unchecked(&mut self, x: usize, y: usize, ch: char, size: f32, color: u32) {
        let glyph = self.font.glyph_id(ch).with_scale(size);
        let glyph = self.font.outline_glyph(glyph).unwrap();
        self.blit_char(x, y, glyph, color)
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