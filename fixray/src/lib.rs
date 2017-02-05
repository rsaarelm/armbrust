#![no_std]

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
