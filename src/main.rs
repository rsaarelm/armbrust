#![feature(lang_items, compiler_builtins_lib, asm)]
#![feature(core_intrinsics)]

#![no_main]
#![no_std]

extern crate compiler_builtins;


pub mod lang_items {
    #[lang = "panic_fmt"]
    #[no_mangle]
    pub extern "C" fn panic_fmt() {}
}

mod stm32f030r8;

use stm32f030r8 as board;


const VGA: *mut board::UsartLayout = board::USART1;

fn putc(uart: *mut board::UsartLayout, c: char) {
    unsafe {
        (*uart).send(c as u16);
    }
}

fn puts(uart: *mut board::UsartLayout, s: &str) {
    for c in s.chars() {
        putc(uart, c);
    }
}

fn getc(uart: *mut board::UsartLayout) -> char {
    unsafe {
        (*uart).recv() as u8 as char
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[derive(Copy, Clone)]
struct Cell {
    c: u8,
    fore: Color,
    back: Color,
}

fn draw_screen<F>(pixel_f: F)
    where F: Fn(u32, u32) -> Color
{
    unimplemented!();
}

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    board::init();

    puts(VGA, "\x1B[H");
    loop {
        putc(VGA, 'a');
    }
}

#[export_name = "_hang"]
pub extern "C" fn hang() -> ! {
    loop {}
}
