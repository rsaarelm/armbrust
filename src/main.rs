#![feature(lang_items, compiler_builtins_lib, asm)]
#![feature(core_intrinsics)]

#![no_main]
#![no_std]

extern crate compiler_builtins;
extern crate fixray;

pub mod lang_items {
    #[lang = "panic_fmt"]
    #[no_mangle]
    pub extern "C" fn panic_fmt() {}
}

mod io;
// mod mandelbrot;
mod stm32f030r8;
mod vga;

use stm32f030r8 as board;

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    board::init();

    let vga = vga::Vga;

    vga.clear();

    use fixray::*;
    use fixray::Color::*;

    let scene = Scene
        + Object::new(sphere_fn(v3(10, 5, 2), fp(3)), m(Material::Surface(Yellow, Red, Black)))
        + Object::new(sphere_fn(v3(5, 10, 2), fp(3)), m(Material::Mirror))
        + Object::new(sphere_fn(v3(0, 15, 2), fp(3)), m(Material::Surface(Yellow, Red, Black)))
        + Object::new(plane_fn(v3(0, 0, 1), fp(0)), checkerboard(Material::Surface(Green, Green, Black), Material::Surface(White, White, Black)))
        ;

    let frustum = Frustum {
        origin: v3(0, 0, 4),
        dir: v3(8, 4, -1).normalized(),
        up: v3(0, 0, 1),
    };

    let light_dir = v3(1, 1, -4).normalized();

    vga.draw_screen(|x, y| {
        trace(&scene, frustum.ray(x, y), &light_dir)
    });

    loop {
    }
}

#[export_name = "_hang"]
pub extern "C" fn hang() -> ! {
    loop {}
}
