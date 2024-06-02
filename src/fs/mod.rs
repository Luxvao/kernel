pub mod drivers;

use crate::{lazy::Lazy, process::ProcessEntry, sync::Mutex};

pub enum FilesystemEntry<'a> {
    TTY(),
    PROC(ProcessEntry<'a>),
    DIRECTORY(),
    FILE(),
}
