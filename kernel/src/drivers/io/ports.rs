use core::arch::asm;

pub unsafe fn inb(port: u16) -> u8 {
    let out: u8;

    asm!(
        "in al, dx",
        in("dx") port,
        out("al") out
    );

    out
}

pub unsafe fn outb(port: u16, val: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
    );
}
