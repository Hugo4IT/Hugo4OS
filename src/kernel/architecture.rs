use super::{memory::MemoryManager, abstractions::rendering::FrameBuffer};

pub trait Architecture: MemoryManager {
    type FrameBuffer: FrameBuffer;
}