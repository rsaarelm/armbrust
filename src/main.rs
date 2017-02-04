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
mod mandelbrot;
mod math;
mod stm32f030r8;
mod vga;

use stm32f030r8 as board;
use vga::Color::*;
use math::{fp, FP};
use geom::{v3, Body, Intersection, Ray, Plane, Sphere};

// We can't have boxed trait objects in core, so hack up an enum instead.
enum BodyType {
    Plane(geom::Plane),
    Sphere(geom::Sphere),
}

impl Body for BodyType {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            &BodyType::Plane(ref a) => a.intersection(ray),
            &BodyType::Sphere(ref a) => a.intersection(ray),
        }
    }
}

#[derive(Copy, Clone)]
enum Material {
    Checkerboard,
    Mirror,
    Shaded,
}

struct Scene {
    // And we have no heap vecs, so hardcode the number of bodies.
    bodies: [(BodyType, Material); 5],
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            bodies: [(BodyType::Plane(geom::Plane {
                         normal: v3(0, 0, 1),
                         offset: fp(0),
                     }),
                      Material::Checkerboard),
                     (BodyType::Sphere(geom::Sphere {
                         center: v3(5, 10, 2),
                         radius: fp(3),
                     }),
                      Material::Shaded),
                     (BodyType::Sphere(geom::Sphere {
                         center: v3(10, 5, 2),
                         radius: fp(3),
                     }),
                      Material::Shaded),
                     (BodyType::Sphere(geom::Sphere {
                         center: v3(-5, 10, 2),
                         radius: fp(3),
                     }),
                      Material::Shaded),
                     (BodyType::Sphere(geom::Sphere {
                         center: v3(-10, 5, 2),
                         radius: fp(3),
                     }),
                      Material::Shaded)],
        }
    }

    pub fn trace(&self, ray: &geom::Ray, reflect_count: usize) -> vga::Color {
        if reflect_count > 10 {
            return Blue;
        }

        let mut dist = FP(i32::max_value());
        let mut hit: Option<(Intersection, &Material)> = None;
        for &(ref b, ref mat) in self.bodies.iter() {
            if let Some(intersect) = b.intersection(&ray) {
                if let Some(prev_hit) = hit {
                    // New hit is further away than existing, ignore.
                    if intersect.distance >= prev_hit.0.distance {
                        continue;
                    }
                }

                // Current best hit, store the material and the intersection.
                hit = Some((intersect, mat));
            }
        }

        if let Some((intersect, mat)) = hit {
            self.resolve(ray, &intersect, mat, reflect_count)
        } else {
            // Hardcoded sky color
            Cyan
        }
    }

    fn resolve(&self, ray: &geom::Ray, intersect: &Intersection, mat: &Material, reflect_count: usize) -> vga::Color {
        match mat {
            &Material::Checkerboard => {
                let hit_pos = ray.origin + ray.dir * intersect.distance;
                // XXX: Hardcoded to hell
                if ((hit_pos.x / fp(8)).to_i32() + (hit_pos.y / fp(8)).to_i32() +
                    (hit_pos.z / fp(8)).to_i32()) % 2 == 0 {
                    White
                } else {
                    Red
                }
            }
            // FIXME: Buggy.
            &Material::Mirror => {
                let reflect = (ray.dir - intersect.normal * (fp(2) * ray.dir.dot(&intersect.normal))).normalized();
                let pos = ray.origin + ray.dir * intersect.distance;
                self.trace(&Ray { origin: pos, dir: reflect }, reflect_count + 1)
            }
            &Material::Shaded => {
                // XXX: Hardcoded to hell.
                let light_dir = v3(1, 1, -2).normalized();
                let angle = intersect.normal.dot(&light_dir);
                if angle < fp(-9) / fp(10) {
                    Yellow
                } else if angle < fp(-7) / fp(10) {
                    Green
                } else if angle < fp(-2) / fp(10) {
                    Blue
                } else {
                    Black
                }
            }
        }
    }
}

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    board::init();

    let vga = vga::Vga;

    vga.clear();

    let scene = Scene::new();

    let frustum = geom::Frustum {
        origin: v3(0, 0, 3),
        dir: v3(1, 2, 0).normalized(),
        up: v3(0, 0, 1),
    };

    vga.draw_screen(|x, y| {
        scene.trace(&frustum.ray(x, y), 0)
    });

    loop {
    }
}

#[export_name = "_hang"]
pub extern "C" fn hang() -> ! {
    loop {}
}
