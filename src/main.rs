#![feature(lang_items)]
#![no_main]
#![no_std]

use core::ops::{Add, Sub, Mul};

mod fixpoint;

use fixpoint::{FP, fp};

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    let sphere = Sphere {
        center: v3(0, 0, 20),
        radius: fp(8)
    };

    let sun_dir = v3(1, -4, 1).normalized();

    for y in -10..10 {
        for x in -20..20 {
            let ray = Ray {
                origin: v3(0, 0, 0),
                dir: v3(x, y, 10).normalized(),
            };

            if let Some(normal) = sphere.intersection(&ray) {
                let light = normal.dir.dot(&sun_dir) * fp(2) + fp(1);
                let gradient = ".,:;i%m#".as_bytes();
                let idx = (light * fp(4)).to_i32();
                let idx = if idx < 0 { 0 } else if idx > 7 { 7 } else { idx } as usize;
                putc(gradient[idx] as char);
            } else {
                putc(' ');
            }
        }
        write("\n");
    }

    write("Hello, world!\n");

    loop {}
}

#[derive(Copy, Clone)]
pub struct V3 {
    x: FP,
    y: FP,
    z: FP,
}

pub fn v3(x: i32, y: i32, z: i32) -> V3 { V3::new(fp(x), fp(y), fp(z)) }

impl V3 {
    pub fn new(x: FP, y: FP, z: FP) -> V3 { V3 { x: x, y: y, z: z } }

    pub fn dot(&self, other: &V3) -> FP { self.x * other.x + self.y * other.y + self.z * other.z }

    pub fn normalized(&self) -> V3 {
        *self * (fp(1) / self.dot(self).sqrt())
    }
}

impl Add for V3 {
    type Output = V3;

    fn add(self, other: V3) -> V3 { V3::new(self.x + other.x, self.y + other.y, self.z + other.z) }
}

impl Sub for V3 {
    type Output = V3;

    fn sub(self, other: V3) -> V3 { V3::new(self.x - other.x, self.y - other.y, self.z - other.z) }
}

impl Mul<FP> for V3 {
    type Output = V3;

    fn mul(self, other: FP) -> V3 { V3::new(self.x * other, self.y * other, self.z * other) }
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
    radius: FP,
}

impl Body for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<Ray> {
        let a = ray.dir.dot(&ray.dir);
        let to_sphere = ray.origin - self.center;
        let b = fp(2) * ray.dir.dot(&to_sphere);
        let c = to_sphere.dot(&to_sphere) - self.radius * self.radius;

        let delta = b * b - fp(4) * a * c;

        if delta <= fp(0) {
            return None;
        }

        let p1 = (-b - delta.sqrt()) / fp(2) * a;
        let p2 = (-b + delta.sqrt()) / fp(2) * a;

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
