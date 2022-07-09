use bootloader::{BootInfo, boot_info::FrameBuffer};

use crate::{util::Locked, kernel::architecture::Architecture};
use memory::FixedSizeBlockAllocator;

pub mod gdt;
pub mod memory;
pub mod rendering;
pub mod interrupts;

#[global_allocator]
pub static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

pub fn init(boot_info: &'static mut BootInfo) -> ! {
    gdt::init();
    interrupts::init();
    interrupts::disable();
    memory::init(
        boot_info.physical_memory_offset.into_option().unwrap(),
        &boot_info.memory_regions
    );

    crate::kernel_main::<X86_64>(boot_info.framebuffer.as_mut().unwrap())
}

pub struct X86_64;
impl Architecture for X86_64 {
    type FrameBuffer = FrameBuffer;
}