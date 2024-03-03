#![no_std]
#![no_main]
#![allow(unused)]

mod gdt;
mod idt;
mod std;
mod syscalls;

#[cfg(any(not(test), rust_analyzer))]
use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() {
    gdt::init();
    idt::init();
}

#[panic_handler]
#[cfg(any(not(test), rust_analyzer))]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
