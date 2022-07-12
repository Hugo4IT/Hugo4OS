use crate::task;

pub trait Interrupts {
    fn enable();
    fn disable();
    fn with_disabled(f: impl FnOnce());
    fn enable_and_halt();
}

use super::abstractions::interrupts::InputSyscall;

/// PIC1 Timer IRQ
pub fn timer() {

}

/// PIC1 Keyboard IRQ
pub fn keyboard(scancode: u8) {
    task::keyboard::add_scancode(scancode);
}

/// PIC2 RealTimeClock IRQ
pub fn rtc() {
    // TODO: Handle RTC IRQ
}

pub fn syscall(args: InputSyscall) -> u64 {
    let target = unsafe { *args.target };


    if target == 1 {
    }

    return 1337;
}