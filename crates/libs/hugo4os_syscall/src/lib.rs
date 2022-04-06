#![no_std]

pub mod arch;

pub unsafe fn test_syscall() {
    let target = 25;
    let arg1 = 69;
    let arg2 = 420;
    let result = syscall!(target, arg1, arg2);

    let print_target = 1;
    syscall!(print_target, result);
}