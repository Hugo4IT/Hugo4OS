use super::{memory::MemoryManager, abstractions::rendering::FrameBuffer, interrupts::Interrupts};

pub trait Architecture: MemoryManager + Interrupts {
    type FrameBuffer: FrameBuffer;
}