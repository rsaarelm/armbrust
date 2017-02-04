#![feature(lang_items, compiler_builtins_lib, asm)]
#![feature(core_intrinsics)]
#![feature(exclusive_range_pattern)]

#![no_main]
#![no_std]

extern crate compiler_builtins;

pub mod lang_items {
    #[lang = "panic_fmt"]
    #[no_mangle]
    pub extern "C" fn panic_fmt() {}
}

mod geom;
mod io;
mod math;
mod stm32f030r8;
mod vga;

use stm32f030r8 as board;
use vga::Color::*;
use math::{fp, FP};

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    board::init();

    let vga = vga::Vga;

    vga.clear();
    vga.draw_screen(|x, y| {
        let x = (fp(x as i32) - fp(48)) / fp(13) / fp(2);
        let y = (fp(y as i32) - fp(32)) / fp(16) / fp(2);
        let m = mandelbrot(x, y);

        match m {
            0 => White,
            1..10 => Blue,
            11..20 => Magenta,
            21..30 => Red,
            31..40 => Green,
            41..50 => Yellow,
            51..99 => Cyan,
            _ => Black,
        }
    });

    loop {
    }
}

fn mandelbrot(cx: FP, cy: FP) -> usize {
    const ITER: usize = 100;
    let mut x = fp(0);
    let mut y = fp(0);

    for i in 0..ITER {
        let x2 = x * x - y * y + cx;
        y = fp(2) * x * y + cy;
        x = x2;

        if x * x + y * y > fp(4) {
            return i;
        }
    }

    return ITER;
}

#[export_name = "_hang"]
pub extern "C" fn hang() -> ! {
    loop {}
}
