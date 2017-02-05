#![no_std]
#![feature(const_fn)]
#![feature(conservative_impl_trait)]

mod fp;
mod geom;
mod scene;
mod v;

pub use fp::{fp, FP};
pub use v::{v3, V3};
pub use geom::{Ray, Frustum};

pub use scene::{Scene, Body, Union, Object, sphere_fn, plane_fn};

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

/// 3-bit display driver.
pub trait Driver {
    fn screen_size(&self) -> (u32, u32);
    fn draw_screen<F>(&self, pixel_f: F) where F: Fn(u32, u32) -> Color;
}

#[derive(Copy, Clone)]
pub enum Material {
    // Perfect reflection.
    Mirror,
    // Highlight, material, shadow
    Surface(Color, Color, Color),
    // XXX: Would be nicer to have the subelements be other Materials, but we'd need boxes for
    // that.
    Checkerboard(Color, Color),
}

pub fn trace<T: Body>(scene: &T, ray: &Ray) -> Color {
    // TODO: Actual raymarcher goes here.
    Color::Magenta
}

/// Wrapper that turns a material into a constant function.
pub fn m(m: Material) -> impl Fn(&V3) -> Material {
    move |_| m
}
