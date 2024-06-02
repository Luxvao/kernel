use crate::{io::port::{inb, outb, ports::COM1}, lazy::Lazy, sync::Mutex};

pub static SERIAL: Lazy<Serial> = Lazy::new(init);

pub struct Serial {
    exists: bool,
}

impl Serial {
    pub fn send_char(&self, value: u8) {
        if !self.exists {
            return;
        }

        while is_transmit_empty() == 0 {}

        outb(COM1, value);
    }

    pub fn send_str(&self, value: &str) {
        if !self.exists {
            return;
        }

        for character in value.chars() {
            self.send_char(character as u8);
        }
    }
}

fn is_transmit_empty() -> u8 {
    inb(COM1 + 5) & 0x20
}

fn init() -> Serial {
    // Disable interrupts
    outb(COM1 + 1, 0);
    // DLAB
    outb(COM1 + 3, 0x80);
    // Set divisor
    outb(COM1, 3);

    outb(COM1 + 1, 0);
    // 8 bits, no parity, one stop bit
    outb(COM1 + 3, 0x03);
    // Enable FIFO, 14 byte threshold
    outb(COM1 + 2, 0xC7);
    // IRQs enabled
    outb(COM1 + 4, 0x0B);
    // Loopback mode
    outb(COM1 + 4, 0x1E);
    // Test serial
    outb(COM1, 0xAE);

    if inb(COM1) != 0xAE {
        return Serial {
            exists: false,
        };
    }

    outb(COM1 + 4, 0x0f);

    Serial {
        exists: true,
    }
}
