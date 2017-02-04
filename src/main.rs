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

mod io;
mod stm32f030r8;
mod vga;

use stm32f030r8 as board;
use vga::Color::*;


#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    board::init();

    let vga = vga::Vga;

    vga.clear();
    vga.draw_screen(|x, y| {
        if x == 0 && y == 0 {
            White
        } else {
            Red
        }
    });

    loop {
    }
}

#[export_name = "_hang"]
pub extern "C" fn hang() -> ! {
    loop {}
}
