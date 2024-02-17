use core::arch::global_asm;

mod handlers;

use handlers::*;

// Assembly wrapper functions around handlers to provide proper ABI
global_asm!(
    "
    .global div_by_zero
    .extern div_by_zero_handler

    div_by_zero:
        pushad
        cld
        call div_by_zero_handler
        popad
        iret
"
);

// Assembly FFI to have access to the wrapper functions
extern "C" {
    pub fn div_by_zero();
}
