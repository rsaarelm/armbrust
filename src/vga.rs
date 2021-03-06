//! Octapentaveega driver

use stm32f030r8 as board;
use io::*;
use fixray::Color;

const PORT: *mut board::UsartLayout = board::USART1;

pub const SCREEN_W: usize = 32;
pub const SCREEN_H: usize = 16;

pub struct Vga;

impl Vga {
    pub fn clear(&self) {
        puts(PORT, "\x1B[2J");
    }

    pub fn wrapon(&self) {
        puts(PORT, "\x1B[?7h");
    }

    pub fn wrapoff(&self) {
        puts(PORT, "\x1B[?7l");
    }

    pub fn home(&self) {
        puts(PORT, "\x1b[H");
    }

    pub fn color(&self, fore: Color, back: Color) {
        puts(PORT, "\x1b[");
        puti(PORT, 30 + fore as i32);
        puts(PORT, ";");
        puti(PORT, 40 + back as i32);
        puts(PORT, "m");
    }

    pub fn pos(&self, x: u32, y: u32) {
        puts(PORT, "\x1b[");
        puti(PORT, x as i32);
        putc(PORT, ';');
        puti(PORT, y as i32);
        putc(PORT, 'H');
    }

    pub fn putc_escaped(&self, c: char) {
        match c as u8 {
            8 | 10 | 13 | 27 | 127 => putc(PORT, '\x1b'),
            _ => {}
        }

        putc(PORT, c);
    }

    pub fn puts(&self, s: &str) {
        puts(PORT, s);
    }

    pub fn graphics_mode(&self, text_up_to: u32) {
        debug_assert!(text_up_to <= 16);
        puts(PORT, "\x1b[");
        puti(PORT, text_up_to as i32);
        puts(PORT, "]");
    }

    pub fn draw_screen<F>(&self, pixel_f: F)
        where F: Fn(u32, u32) -> Color
    {
        self.graphics_mode(16);
        puts(PORT, "\x1bG"); // Tricoder mode start

        // Iterate through the screen characters.
        for char_offset in 0..(SCREEN_W * SCREEN_H) {
            if char_offset == SCREEN_W * SCREEN_H - 1 {
                // Disable wrapping out of screen.
                self.wrapoff();
            }

            let x = ((char_offset % SCREEN_W) * 2) as u32;
            let y = ((char_offset / SCREEN_W) * 4) as u32;
            // Collect the pixels that the current character slot will cover.
            let pixels = [pixel_f(x + 1, y + 3) as u8,
                          pixel_f(x, y + 3) as u8,
                          pixel_f(x + 1, y + 2) as u8,
                          pixel_f(x, y + 2) as u8,
                          pixel_f(x + 1, y + 1) as u8,
                          pixel_f(x, y + 1) as u8,
                          pixel_f(x + 1, y) as u8,
                          pixel_f(x, y) as u8];

            // Construct the rgb masks.
            let mut r = 0u8;
            let mut g = 0u8;
            let mut b = 0u8;

            for i in 0..8 {
                if pixels[i] & 0b001 != 0 {
                    r |= 1 << i;
                }
                if pixels[i] & 0b010 != 0 {
                    g |= 1 << i;
                }
                if pixels[i] & 0b100 != 0 {
                    b |= 1 << i;
                }
            }

            self.putc_escaped(r as char);
            self.putc_escaped(g as char);
            self.putc_escaped(b as char);

        }

        self.wrapon();

        puts(PORT, "\x1bT"); // Tricoder mode end
    }
}
