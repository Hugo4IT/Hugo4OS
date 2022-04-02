use fontdue::{Font, FontSettings, Metrics};

pub struct FontRasterizer {
    font: Font,
}

impl FontRasterizer {
    pub fn new(font: &[u8]) -> FontRasterizer {
        FontRasterizer {
            font: Font::from_bytes(font, FontSettings::default()).unwrap(),
        }
    }

    pub fn rasterize(&mut self, ch: char, size: f32) -> (Metrics, &[u32]) {
        
    }
}