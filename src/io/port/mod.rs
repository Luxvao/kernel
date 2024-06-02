use core::arch::asm;

pub mod ports;

pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
        );
    }
}

pub fn inb(port: u16) -> u8 {
    let ret;

    unsafe {
        asm!(
        "in al, dx",
        in("dx") port,
        out("al") ret,
        );
    }

    ret
}
