use bootloader::BootInfo;

use crate::util::Locked;
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

    crate::kernel_main(boot_info.framebuffer.as_mut().unwrap())
}