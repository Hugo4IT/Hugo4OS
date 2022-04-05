use crate::task;

#[cfg(target_arch = "x86_64")] pub use crate::arch::_x86_64::interrupts::enable as enable;
#[cfg(target_arch = "x86_64")] pub use crate::arch::_x86_64::interrupts::disable as disable;
#[cfg(target_arch = "x86_64")] pub use crate::arch::_x86_64::interrupts::with_disabled as with_disabled;

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