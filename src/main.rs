#![feature(lang_items)]
#![no_main]
#![no_std]

use core::ops::{Add, Sub, Mul};

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    write("Hello, world!\n");

    loop {}
}

#[derive(Copy, Clone, Debug)]
pub struct V3 {
    x: f32,
    y: f32,
    z: f32,
}

// XXX: Rust sqrt method is in std lib, not available here.
pub fn sqrt(x: f32) -> f32 {
    unimplemented!();
}

pub fn v3(x: f32, y: f32, z: f32) -> V3 { V3 { x: x, y: y, z: z } }

impl V3 {
    pub fn dot(&self, other: &V3) -> f32 { self.x * other.x + self.y * other.y + self.z * other.z }

    pub fn normalized(&self) -> V3 {
        *self * (1.0 / sqrt(self.dot(self)))
    }
}

impl Add for V3 {
    type Output = V3;

    fn add(self, other: V3) -> V3 { v3(self.x + other.x, self.y + other.y, self.z + other.z) }
}

impl Sub for V3 {
    type Output = V3;

    fn sub(self, other: V3) -> V3 { v3(self.x - other.x, self.y - other.y, self.z - other.z) }
}

impl Mul<f32> for V3 {
    type Output = V3;

    fn mul(self, other: f32) -> V3 { v3(self.x * other, self.y * other, self.z * other) }
}

struct Ray {
    origin: V3,
    dir: V3,
}

trait Body {
    fn intersection(&self, ray: &Ray) -> Option<Ray>;
}

struct Sphere {
    center: V3,
    radius: f32,
}

impl Body for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<Ray> {
        let a = ray.dir.dot(&ray.dir);
        let to_sphere = ray.origin - self.center;
        let b = 2.0 * ray.dir.dot(&to_sphere);
        let c = to_sphere.dot(&to_sphere) - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;

        if delta <= 0.0 {
            return None;
        }

        let p1 = (-b - sqrt(delta)) / 2.0 * a;
        let p2 = (-b + sqrt(delta)) / 2.0 * a;

        let p = if p1 < p2 { p1 } else { p2 };

        let pos = ray.origin + ray.dir * p;
        let normal = (pos - self.center).normalized();

        Some(Ray {
            origin: pos,
            dir: normal,
        })
    }
}



pub fn write(text: &str) {
    for c in text.chars() { putc(c); }
}

#[inline]
pub fn putc(c: char) {
    const UART0: u32 = 0x4000C000;
    unsafe { *(UART0 as *mut u32) = c as u32 }
}

mod lang_items {
    #[lang = "panic_fmt"]
    extern "C" fn panic_fmt() {}
}
