use core::arch::asm;

// Functions
pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("cli", "hlt");
        }
    }
}
