#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader::BootInfo;

use hugo4os::kernel::architecture::Architecture;
use memory::FixedSizeBlockAllocator;
use rendering::FrameBuffer;

pub mod gdt;
pub mod memory;
pub mod rendering;
pub mod interrupts;

#[global_allocator]
pub static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

bootloader::entry_point!(init);

pub fn init(boot_info: &'static mut BootInfo) -> ! {
    gdt::init();
    interrupts::init();
    interrupts::disable();
    memory::init(
        boot_info.physical_memory_offset.into_option().unwrap(),
        &boot_info.memory_regions
    );

    hugo4os::kernel_main::<X86_64>(FrameBuffer::new(boot_info.framebuffer.as_ref().unwrap()))
}

pub struct X86_64;
impl Architecture for X86_64 {
    type FrameBuffer = FrameBuffer;
}

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
#[cfg(test)]
fn panic(_info: &PanicInfo) -> ! {
    println!("Failed!");
    println!("Error: {}", _info);
    tests::exit_qemu(0x10);
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", _layout);
}

#[cfg(feature = "serial")] use core::fmt;

#[cfg(feature = "serial")] use lazy_static::lazy_static;
#[cfg(feature = "serial")] use uart_16550::SerialPort;

#[cfg(feature = "serial")] lazy_static! {
    pub static ref SERIAL1: spin::Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        spin::Mutex::new(serial_port)
    };
}

#[macro_export]
macro_rules! println_verbose {
    () => {
        #[cfg(feature = "verbose")]
        $crate::println!("\n");
    };
    ($($arg:tt)*) => {
        #[cfg(feature = "verbose")]
        $crate::print!("{}:{} [{}] {}\n", file!(), line!(), module_path!(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! print_verbose {
    ($($arg:tt)*) => {
        #[cfg(feature = "verbose")]
        $crate::print!("{}:{} [{}] {}", file!(), line!(), module_path!(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println_debug {
    () => {
        #[cfg(debug_assertions)]
        println!("\n");
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::println!($($arg)*);
    };
}

#[macro_export]
macro_rules! print_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::print!($($arg)*);
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)))
}

#[cfg(feature = "serial")]
#[macro_export] macro_rules! print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}
#[cfg(not(feature = "serial"))]
#[macro_export] macro_rules! print {
    ($($arg:tt)*) => {()};
}

#[doc(hidden)]
#[cfg(feature = "serial")]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).expect("Printing to serial failed!");
    })
}

/// Wrapper to add trait implementation support to spin::Mutex.
pub struct Locked<A>(spin::Mutex<A>);
impl<A> Locked<A> {
    pub const fn new(inner: A) -> Locked<A> { Locked(spin::Mutex::new(inner)) }
    pub fn lock(&self) -> spin::MutexGuard<A> { self.0.lock() }
}

// Tests

pub fn exit_qemu(exit_code: u32) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for (_i, test) in tests.iter().enumerate() {
        test.run();
    }

    println!("\nThis is a false positive:");
    exit_qemu(0x11);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("[{}] Running... ", core::any::type_name::<T>());
        self();
        println!("Ok!");
    }
}

#[test_case]
fn check_test_framework() {
    assert_eq!(1, 1);
}

// Interrupts

#[test_case]
fn check_crash_catch() {
    x86_64::instructions::interrupts::int3();
}