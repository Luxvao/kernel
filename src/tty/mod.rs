use crate::io::{stdin::Stdin, stdout::Stdout};

const DEFAULT_WIDTH: usize = 1920;
const DEFAULT_HEIGHT: usize = 1080;

pub struct TTYEntry {
    pub id: usize,
    pub active: bool,
    pub tty: TTY,
}

pub struct TTY {
    pub stdin: Stdin,
    pub stdout: Stdout,
    inner_buffer: [[char; DEFAULT_WIDTH]; DEFAULT_HEIGHT],
}
