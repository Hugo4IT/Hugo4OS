
use alloc::vec::Vec;

use crate::raw;

pub trait StreamWrite {
    fn write(&mut self, data: &[u8]);
}

pub trait StreamRead {
    fn read(&mut self, count: usize) -> Vec<u8>;
}

pub struct Stream {
    id: u64,
}

impl Stream {
    pub fn new() -> Stream {
        Stream {
            id: unsafe { raw::stream_create() },
        }
    }
}

impl StreamWrite for Stream {
    fn write(&mut self, data: &[u8]) {
        unsafe {
            raw::stream_write(self.id, &data[0] as *const u8 as u64, data.len() as u64);
        }
    }
}

impl StreamRead for Stream {
    fn read(&mut self, count: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(count);
        unsafe {
            raw::stream_read(self.id, buffer.as_mut_ptr() as u64, count as u64);
        }

        buffer
    }
}

pub struct BufferedStream<const SIZE: usize> {
    inner: Stream,
    buffer: [u8; SIZE],
}

impl<const SIZE: usize> BufferedStream<SIZE> {
    pub fn new() -> BufferedStream<SIZE> {
        BufferedStream {
            inner: Stream::new(),
            buffer: [0; SIZE],
        }
    }

    pub fn flush(&mut self) {
        self.inner.write(&self.buffer)
    }
}

impl<const SIZE: usize> StreamRead for BufferedStream<SIZE> {
    fn read(&mut self, count: usize) -> Vec<u8> {
        self.inner.read(count)
    }
}

impl<const SIZE: usize> StreamWrite for BufferedStream<SIZE> {
    fn write(&mut self, data: &[u8]) {
        self.inner.write(data)
    }
}