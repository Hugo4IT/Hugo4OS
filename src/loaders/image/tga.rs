use core::slice::Iter;

use alloc::vec::Vec;

use super::Image;

#[derive(Debug, Clone, Copy)]
pub enum TGAImageParsingError {
    InvalidMagicNumber,
    UnsupportedEncoding,
    UnexpectedEOF,
}

#[allow(dead_code)]
struct TGAImageHeader {
    pub magic1: u8,
    pub colormap: u8,
    pub encoding: u8,
    pub cmaporig: u16,
    pub cmaplen: u16,
    pub cmapent: u8,
    pub x: u16,
    pub y: u16,
    pub h: u16,
    pub w: u16,
    pub bpp: u8,
    pub pixeltype: u8,
}

impl TGAImageHeader {
    pub fn from_byte_iterator<'a>(iterator: &mut Iter<'a, u8>) -> Result<TGAImageHeader, TGAImageParsingError> {
        let magic1 = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        let colormap = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        let encoding = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        if encoding != 2 {
            return Err(TGAImageParsingError::UnsupportedEncoding)
        }
        let cmaporig = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        let cmaplen = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        let cmapent = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        let x = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        let y = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        let h = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        let w = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        let bpp = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        let pixeltype = iterator.parse(TGAImageParsingError::UnexpectedEOF)?;
        
        Ok(TGAImageHeader {
            magic1,
            colormap,
            encoding,
            cmaporig,
            cmaplen,
            cmapent,
            x,
            y,
            h,
            w,
            bpp,
            pixeltype,
        })
    }
}

pub struct TGAImageFile {
    header: TGAImageHeader,
    data: Vec<u32>,
}

impl TGAImageFile {
    pub fn from_bytes(bytes: &[u8]) -> Result<TGAImageFile, TGAImageParsingError> {
        let mut iterator = bytes.iter();
        let header = TGAImageHeader::from_byte_iterator(&mut iterator)?;
        let data = iterator
            .map(|b|*b)
            .collect::<Vec<u8>>()
            .chunks(4)
            .into_iter()
            .map(|p|u32::from_be_bytes([p[3], p[2], p[1], p[0]]))
            .collect::<Vec<u32>>();

        Ok(TGAImageFile {header, data})
    }
}

impl Image for TGAImageFile {
    #[inline] fn get_width(&self) -> usize {
        self.header.w as usize
    }
    
    #[inline] fn get_height(&self) -> usize {
        self.header.h as usize
    }

    #[inline] fn get_texture(&self) -> Vec<u32> {
        self.data.clone()
    }
}

trait Parse<T, E> {
    fn parse(&mut self, error: E) -> Result<T, E>;
    unsafe fn parse_unchecked(&mut self) -> T;
}

impl<'a, E> Parse<u8, E> for Iter<'a, u8> {
    fn parse(&mut self, error: E) -> Result<u8, E> {
        self.next().map_or_else(|| Err(error), |b|Ok(*b))
    }

    unsafe fn parse_unchecked(&mut self) -> u8 {
        *self.next().unwrap_unchecked()
    }
}

impl<'a, E: Copy> Parse<u16, E> for Iter<'a, u8> {
    fn parse(&mut self, error: E) -> Result<u16, E> {
        let mut bytes: [u8; 2] = [0; 2];
        bytes[0] = self.next().map_or_else(|| Err(error), |b|Ok(*b))?;
        bytes[1] = self.next().map_or_else(|| Err(error), |b|Ok(*b))?;
        
        Ok(u16::from_le_bytes(bytes))
    }

    unsafe fn parse_unchecked(&mut self) -> u16 {
        let mut bytes: [u8; 2] = [0; 2];
        bytes[0] = *self.next().unwrap_unchecked();
        bytes[1] = *self.next().unwrap_unchecked();
        
        u16::from_le_bytes(bytes)
    }
}