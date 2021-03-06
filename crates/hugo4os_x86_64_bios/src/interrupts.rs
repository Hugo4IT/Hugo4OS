use hugo4os::kernel::interrupts::Interrupts;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::instructions::port::Port;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;

use hugo4os::kernel::{abstractions, self};

pub use x86_64::instructions::interrupts::enable_and_hlt;
pub use x86_64::instructions::interrupts::without_interrupts as with_disabled;
pub use x86_64::instructions::interrupts::disable;
pub use x86_64::instructions::interrupts::enable;

use crate::{println, X86_64};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard_handler);
        idt[InterruptIndex::RealTimeClock as usize].set_handler_fn(realtime_clock_handler);
        idt[InterruptIndex::Syscall as usize].set_handler_fn(syscall_handler);

        idt
    };
}

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

impl Interrupts for X86_64 {
    #[inline]
    fn enable() {
        enable()
    }

    #[inline]
    fn disable() {
        disable()
    }

    #[inline]
    fn with_disabled(f: impl FnOnce()) {
        with_disabled(f)
    }

    #[inline]
    fn enable_and_halt() {
        enable_and_hlt()
    }
}

pub fn init() {
    IDT.load();
    unsafe { PICS.lock().initialize(); }

    // Enable the RealTimeClock (defaults to 1024Hz)
    let mut rtc_register_selector = Port::new(0x70);
    let mut rtc_configuration = Port::new(0x71);

    unsafe {
        rtc_register_selector.write(0x8Bu8);
        let prev: u8 = rtc_configuration.read();
        rtc_register_selector.write(0x8Bu8); // A read resets index, so write again
        rtc_configuration.write(prev | 0x40u8);
    };

    enable();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,           // Handled
    Keyboard,                       // Handled
    SecondaryPIC,
    SerialPort2,
    SerialPort1,
    ParallelPort2_3,
    FloppyDisk,
    ParallelPort1,

    // Start second PIC

    RealTimeClock = PIC_2_OFFSET,   // Handled
    ACPI,
    Available1,
    Available2,
    Mouse,                          // TODO: Handle mouse IRQ
    CoProcessor,
    PrimaryATA,
    SecondaryATA,

    // Start custom interrupts

    Syscall = 0x80,                 // Same address as linux for compatability
}

// Exceptions

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", _stack_frame);
}

extern "x86-interrupt" fn simd_floating_point_handler(_stack_frame: InterruptStackFrame) {
    println!("SIMD EXCEPTION: {:#?}", _stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(_stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}\n", _stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", x86_64::registers::control::Cr2::read());
    println!("Error Code: {:?}", _error_code);
    println!("{:#?}", _stack_frame);
}

// Interrupt vectors

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    kernel::interrupts::timer();

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    kernel::interrupts::keyboard(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}

extern "x86-interrupt" fn realtime_clock_handler(_stack_frame: InterruptStackFrame) {
    kernel::interrupts::rtc();

    let mut r_port = Port::new(0x70);
    let mut d_port = Port::new(0x71);

    unsafe {
        r_port.write(0x0Cu8);

        // RTC Won't fire again if you dont read the C register
        let _: u8 = d_port.read();
        
        PICS.lock().notify_end_of_interrupt(InterruptIndex::RealTimeClock as u8)
    }
}

/// The x86_64 implementation of syscalls
/// 
/// Receives `int 0x80` (most probably called from [`hugo4os_syscall::syscall!`]).
/// It then puts all the parameters (stored in 12 CPU registers) in variables
/// and executes [`kernel::interrupts::syscall`] (abstraction). It then takes the
/// return value from that function, and writes it to the pointer specified in
/// the `eax` register (also used to specify which syscall to run).
extern "x86-interrupt" fn syscall_handler(_stack_frame: InterruptStackFrame) {
    let regs = unsafe {
        let mut r15: u64;
        let mut r14: u64;
        let mut r13: u64;
        let mut r12: u64;
        let mut r11: u64;
        let mut r10: u64;
        let mut r9: u64;
        let mut r8: u64;
        let mut eax: u64;
        let mut rcx: u64;
        let mut rdx: u64;
        let mut rsi: u64;
        core::arch::asm!(
            "",
            out("rsi") rsi,
            out("rdx") rdx,
            out("rcx") rcx,
            out("eax") eax, // Specifies which syscall to run, and will be reused as a pointer to return value
            out("r8") r8,
            out("r9") r9,
            out("r10") r10,
            out("r11") r11,
            out("r12") r12,
            out("r13") r13,
            out("r14") r14,
            out("r15") r15,
            options(nomem, nostack)
        );
        let eax = eax as *mut u64;

        SyscallRegs { r15, r14, r13, r12, r11, r10, r9, r8, eax, rcx, rdx, rsi }
    };

    let result = kernel::interrupts::syscall(regs.into());
    unsafe { core::ptr::write(regs.eax, result) }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SyscallRegs {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub eax: *mut u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
}

impl Into<abstractions::interrupts::InputSyscall> for SyscallRegs {
    fn into(self) -> abstractions::interrupts::InputSyscall {
        abstractions::interrupts::InputSyscall {
            args: [
                self.rsi,
                self.rdx,
                self.rcx,
                self.r8,
                self.r9,
                self.r10,
                self.r11,
                self.r12,
                self.r13,
                self.r14,
                self.r15,
            ],
            target: self.eax,
        }
    }
}