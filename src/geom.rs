use core::ops::{Add, Sub, Mul, Div, Neg};
use math::{fp, FP};

#[derive(Copy, Clone)]
pub struct V3 {
    x: FP,
    y: FP,
    z: FP,
}

pub fn v3(x: i32, y: i32, z: i32) -> V3 {
    V3::new(fp(x), fp(y), fp(z))
}

impl V3 {
    pub fn new(x: FP, y: FP, z: FP) -> V3 {
        V3 { x: x, y: y, z: z }
    }

    pub fn dot(&self, other: &V3) -> FP {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalized(&self) -> V3 {
        *self * (fp(1) / self.dot(self).sqrt())
    }
}

impl Add for V3 {
    type Output = V3;

    fn add(self, other: V3) -> V3 {
        V3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for V3 {
    type Output = V3;

    fn sub(self, other: V3) -> V3 {
        V3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<FP> for V3 {
    type Output = V3;

    fn mul(self, other: FP) -> V3 {
        V3::new(self.x * other, self.y * other, self.z * other)
    }
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

        let p = if p1 < p2 {
            p1
        } else {
            p2
        };

        let pos = ray.origin + ray.dir * p;
        let normal = (pos - self.center).normalized();

        Some(Ray {
            origin: pos,
            dir: normal,
        })
    }
}
