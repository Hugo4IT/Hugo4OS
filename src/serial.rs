use core::fmt;

use lazy_static::lazy_static;
use uart_16550::SerialPort;

lazy_static! {
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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).expect("Printing to serial failed!");
    })
}