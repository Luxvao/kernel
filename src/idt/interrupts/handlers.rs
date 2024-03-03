use crate::{
    idt::interrupts::SYSTEMCALLRESPONSE,
    syscalls::{
        definitions::{SystemCall, SystemCallResponse},
        handler,
    },
};
use core::arch::asm;

// Interrupt handlers
#[no_mangle]
extern "C" fn div_by_zero_handler() {}

#[no_mangle]
extern "C" fn syscall_handler() {
    let syscall: *const SystemCall;

    unsafe {
        asm!(
            "mov {syscall}, eax",
            syscall = out(reg) syscall,
        );

        let response = handler(&*syscall) as *const SystemCallResponse;

        asm!(
            "mov {SYSTEMCALLRESPONSE}, {response}",
            response = in(reg) response,
            SYSTEMCALLRESPONSE = sym SYSTEMCALLRESPONSE
        );
    }
}
