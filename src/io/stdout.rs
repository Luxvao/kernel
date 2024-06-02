const DEFAULT_STDOUT_SIZE: usize = 1000;

pub struct Stdout {
    pointer: usize,
    buffer: [char; DEFAULT_STDOUT_SIZE],
}

impl Stdout {
    pub fn new() -> Stdout {
        Stdout {
            pointer: 0,
            buffer: ['\0'; DEFAULT_STDOUT_SIZE],
        }
    }

    pub fn push(&mut self, input: char) {
        if self.pointer > DEFAULT_STDOUT_SIZE {
            self.stdout_overflow();
        }

        self.buffer[self.pointer] = input;

        self.pointer += 1;
    }

    pub fn get(&self) -> Option<char> {
        Some(self.buffer[self.pointer.checked_sub(1)?])
    }

    fn stdout_overflow(&mut self) {
        for character in self.buffer.iter_mut() {
            *character = '\0';
        }

        self.pointer = 0;
    }
}
