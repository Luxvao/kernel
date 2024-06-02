use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use crate::{
    framebuffer::{Color, FRAMEBUFFER},
    utils::hcf,
};

pub extern "x86-interrupt" fn divide_error_handler(_frame: InterruptStackFrame) {
    hcf();
}

pub extern "x86-interrupt" fn page_fault_handler(
    _frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    hcf();
}
