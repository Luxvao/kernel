use x86_64::structures::idt::{Entry, HandlerFunc, InterruptDescriptorTable, PageFaultHandlerFunc};

use crate::idt::interrupt::{divide_error_handler, page_fault_handler};

pub mod interrupt;

pub fn init() {
    static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

    unsafe {
        // Handler for division errors
        let mut div_error_entry: Entry<HandlerFunc> = Entry::missing();

        div_error_entry.set_handler_fn(divide_error_handler);

        IDT.divide_error = div_error_entry;

        // Handler for page faults
        let mut page_fault_entry: Entry<PageFaultHandlerFunc> = Entry::missing();

        page_fault_entry.set_handler_fn(page_fault_handler);

        IDT.page_fault = page_fault_entry;

        // Loads the IDT
        IDT.load();
    }
}
