#![no_std]
#![feature(const_fn)]

mod fp;
mod geom;
mod v;

pub use fp::{fp, FP};
pub use v::{v3, V3};

/// 3-bit color.
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

pub trait Driver {
    fn screen_size(&self) -> (u32, u32);
    fn draw_screen<F>(&self, pixel_f: F) where F: Fn(u32, u32) -> Color;
}

pub trait Body {
    /// Signed distance function for the surface of the body.
    fn distance(&self, pos: &V3) -> FP;

    fn material(&self, pos: &V3) -> Material {
        Material::Surface(Color::Yellow, Color::Green, Color::Blue)
    }

    fn normal(&self, pos: &V3) -> V3 {
        pos.grad(|p| self.distance(&p))
    }
}



pub enum Material {
    // Perfect reflection.
    Mirror,
    // Highlight, material, shadow
    Surface(Color, Color, Color),
    // XXX: Would be nicer to have the subelements be other Materials, but we'd need boxes for
    // that.
    Checkerboard(Color, Color),
}

pub trait Scene {

}
