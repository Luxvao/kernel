pub mod bitmap;

use core::sync::atomic::{AtomicPtr, Ordering};

use limine::request::FramebufferRequest;

use crate::{lazy::Lazy, sync::Mutex, utils::hcf};

pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub static FRAMEBUFFER: Lazy<Mutex<FramebufferInfo>> = Lazy::new(get_framebuffer);

pub struct FramebufferInfo {
    buffer: AtomicPtr<u8>,
    width: usize,
    height: usize,
    pixelwidth: usize,
    pitch: usize,
}

#[derive(Copy, Clone)]
pub struct Color(u8, u8, u8);

impl FramebufferInfo {
    pub fn putpixel(&self, x: usize, y: usize, color: Color) {
        if x > self.width || y > self.height {
            return;
        }

        let pos = x * self.pixelwidth + y * self.pitch;

        unsafe {
            *self.buffer.load(Ordering::Relaxed).add(pos) = color.0;
            *self.buffer.load(Ordering::Relaxed).add(pos + 1) = color.1;
            *self.buffer.load(Ordering::Relaxed).add(pos + 2) = color.2;
        }
    }

    pub fn drawrect(&self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        for y in y..y + h {
            for x in x..x + w {
                self.putpixel(x, y, color);
            }
        }
    }

    pub fn clrscr(&self, color: Color) {
        self.drawrect(0, 0, self.width, self.height, color);
    }
}

impl Color {
    pub fn new() -> Self {
        Color(0, 0, 0)
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Color((value) as u8, (value >> 8) as u8, (value >> 16) as u8)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Color(value.0, value.1, value.2)
    }
}

fn get_framebuffer() -> Mutex<FramebufferInfo> {
    if let Some(framebuffer_resp) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_resp.framebuffers().next() {
            let info = FramebufferInfo {
                buffer: AtomicPtr::new(framebuffer.addr()),
                width: framebuffer.width() as usize,
                height: framebuffer.height() as usize,
                pixelwidth: framebuffer.bpp() as usize / 8,
                pitch: framebuffer.pitch() as usize,
            };

            return Mutex::new(info);
        }
    }

    hcf();
}
