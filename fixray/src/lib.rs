#![no_std]
#![feature(const_fn)]

mod fp;
mod geom;
mod v;

pub use fp::{fp, FP};
pub use v::{v3, V3};

use core::cmp::min;
use core::ops::Add;

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

pub struct Union<T, U> {
    first: T,
    second: U,
}

impl<T: Body, U: Body> Body for Union<T, U> {
    fn distance(&self, pos: &V3) -> FP {
        min(self.first.distance(pos), self.second.distance(pos))
    }

    fn material(&self, pos: &V3) -> Material {
        if self.first.distance(pos) < self.second.distance(pos) {
            self.first.material(pos)
        } else {
            self.second.material(pos)
        }
    }

    fn normal(&self, pos: &V3) -> V3 {
        // Overriding this so that it'll fall into the cheaper component method before starting the
        // expensive gradient operation.
        if self.first.distance(pos) < self.second.distance(pos) {
            self.first.normal(pos)
        } else {
            self.second.normal(pos)
        }
    }
}

impl <T: Body, U: Body, V: Body> Add<V> for Union<T, U> {
    type Output = Union<Union<T, U>, V>;

    fn add(self, other: V) -> Self::Output {
        Union {
            first: self,
            second: other,
        }
    }
}

// Dummy type to start a scene.
pub struct Scene;

impl Body for Scene {
    fn distance(&self, pos: &V3) -> FP {
        FP(i32::max_value())
    }

    fn material(&self, pos: &V3) -> Material {
        Material::Surface(Color::Cyan, Color::Cyan, Color::Cyan)
    }
}

impl <T: Body> Add<T> for Scene {
    type Output = Union<Scene, T>;

    fn add(self, other: T) -> Self::Output {
        Union {
            first: self,
            second: other,
        }
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
