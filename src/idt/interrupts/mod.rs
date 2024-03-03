mod handlers;

use core::arch::global_asm;
use handlers::*;

#[no_mangle]
static SYSTEMCALLRESPONSE: u32 = 0;

// Assembly wrapper functions around handlers to provide proper protection
global_asm!(
    "
    .global div_by_zero
    .extern div_by_zero_handler

    .global syscall
    .extern syscall_handler

    div_by_zero:
        pushad
        cld
        call div_by_zero_handler
        popad
        iret

    syscall:
        pushad
        cld
        call syscall_handler
        popad
        mov eax, {SYSTEMCALLRESPONSE}
        iret
    ",
    SYSTEMCALLRESPONSE = sym SYSTEMCALLRESPONSE,
);

// Assembly FFI to have access to the wrapper functions
extern "C" {
    pub fn div_by_zero();
    pub fn syscall();
}
