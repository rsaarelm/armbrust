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
}

pub fn checkerboard(m1: Material, m2: Material) -> impl Fn(&V3) -> Material {
    // XXX: Arbitrary scale here, could do that with another fn?
    move |&p| {
        if ((p.x / fp(8)).to_i32() + (p.y / fp(8)).to_i32() +
            (p.z / fp(8)).to_i32()) % 2 == 0 {
            m1
        } else {
            m2
        }
    }
}

pub fn trace<T: Body>(body: &T, mut ray: Ray, light_dir: &V3) -> Color {
    const MAX_STEPS: usize = 256;
    const ESCAPE_VELOCITY: FP = fp(1000);
    const SURFACE_DEFLECT: FP = FP(8);

    let mut advance = fp(0);
    let mut n_steps = 0;

    let mut is_mirror = false;

    loop {
        let pos = ray.origin + ray.dir * advance;
        let d = body.distance(&pos);

        if d <= FP(0) {
            let normal = body.normal(&pos);

            // TODO: Shadow when light source is blocked, needs a second type of trace function
            // that just checks for path.
            match body.material(&pos) {
                Material::Mirror => {
                    let reflect = (ray.dir - normal * (fp(2) * ray.dir.dot(&normal))).normalized();
                    // Deflect a bit off the surface so we don't get stuck inside it...
                    let pos = pos + normal * SURFACE_DEFLECT;
                    ray = Ray { origin: pos, dir: reflect };
                    is_mirror = true;
                }
                Material::Surface(highlight, col, shadow) => {
                    let light_angle = light_dir.dot(&normal);
                    return if light_angle < fp(-8) / fp(16) {
                        highlight
                    } else if light_angle < fp(0) {
                        col
                    } else {
                        shadow
                    };
                }
            }
        }

        if advance > ESCAPE_VELOCITY {
            break;
        }

        advance = advance + d;

        n_steps += 1;
        if n_steps > MAX_STEPS {
            return Color::Black;
        }
    }

    // Hack things a bit to show a blue sky in mirrors so they aren't quite that invisible.
    if is_mirror {
        Color::Blue
    } else {
        Color::Cyan
    }
}

/// Wrapper that turns a material into a constant function.
pub fn m(m: Material) -> impl Fn(&V3) -> Material {
    move |_| m
}
