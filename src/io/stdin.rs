const DEFAULT_STDIN_SIZE: usize = 1000;

pub struct Stdin {
    pointer: usize,
    buffer: [char; DEFAULT_STDIN_SIZE],
}

impl Stdin {
    pub fn new() -> Stdin {
        Stdin {
            pointer: 0,
            buffer: ['\0'; DEFAULT_STDIN_SIZE],
        }
    }

    pub fn push(&mut self, input: char) {
        if self.pointer > DEFAULT_STDIN_SIZE {
            self.stdin_overflow();
        }

        self.buffer[self.pointer] = input;

        self.pointer += 1;
    }

    pub fn get(&self) -> Option<char> {
        Some(self.buffer[self.pointer.checked_sub(1)?])
    }

    fn stdin_overflow(&mut self) {
        for character in self.buffer.iter_mut() {
            *character = '\0';
        }

        self.pointer = 0;
    }
}
