use bootloader::BootInfo;

pub mod interrupts;
pub mod rendering;
pub mod memory;
pub mod gdt;

pub trait Kernel {
    fn init(&mut self, boot_info: &'static BootInfo);
}