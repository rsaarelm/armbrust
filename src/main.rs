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

mod geom;
mod io;
mod math;
mod stm32f030r8;
mod vga;

use stm32f030r8 as board;
use vga::Color::*;
use math::{fp, FP};
use geom::{v3, Body};

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    board::init();

    let vga = vga::Vga;

    vga.clear();

    let frustum = geom::Frustum {
        origin: v3(0, 0, 3),
        dir: v3(1, 2, 0).normalized(),
        up: v3(0, 0, 1),
    };

    let ground = geom::Plane {
        normal: v3(0, 0, 1),
        offset: fp(0),
    };

    vga.draw_screen(|x, y| {
        let ray = frustum.ray(x, y);

        if let Some(r) = ground.intersection(&ray) {
            if (r.origin.x / fp(8)).to_i32() % 2 != (r.origin.y / fp(8)).to_i32() % 2 {
                Yellow
            } else {
                Green
            }
        } else {
            Cyan
        }
    });

    loop {
    }
}

fn draw_mandelbrot(x: u32, y: u32) -> vga::Color {
    let x = (fp(x as i32) - fp(48)) / fp(13) / fp(2);
    let y = (fp(y as i32) - fp(32)) / fp(16) / fp(2);
    let m = mandelbrot(x, y);

    if m == 0 {
        White
    } else if m < 10 {
        Blue
    } else if m < 20 {
        Magenta
    } else if m < 30 {
        Red
    } else if m < 40 {
        Green
    } else if m < 50 {
        Yellow
    } else if m < 99 {
        Cyan
    } else {
        Black
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
