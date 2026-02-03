use core::fmt::Write;

use crate::drivers::io::ports::{inb, outb};

#[derive(Clone, Copy, Debug)]
pub enum UartPort {
    COM1 = 0x3F8,
}

#[derive(Clone, Copy, Debug)]
pub struct Uart {
    port: UartPort,
}

impl Uart {
    pub fn init(port: UartPort) -> Uart {
        let uart = Uart { port };

        let port = port as u16;

        unsafe {
            outb(port + 3, 0x80);

            outb(port + 0, 0x01);
            outb(port + 1, 0x00);

            outb(port + 3, 0x03);

            outb(port + 2, 0xC7);

            outb(port + 1, 0x00);
        }

        uart
    }

    pub fn write_byte(&self, byte: u8) {
        unsafe {
            while inb(self.port as u16 + 5) & 0x20 == 0 {}
            outb(self.port as u16, byte);
        }
    }

    pub fn write_bytes(&self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_bytes(s.as_bytes());

        Ok(())
    }
}
