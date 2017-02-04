use core::ops::{Add, Sub, Mul, Div, Neg};
use math::{fp, FP};

#[derive(Copy, Clone)]
pub struct V3 {
    pub x: FP,
    pub y: FP,
    pub z: FP,
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

    pub fn cross(&self, other: &V3) -> V3 {
        V3::new(self.y * other.z - self.z * other.y,
                self.z * other.x - self.x * other.z,
                self.x * other.y - self.y * other.x)
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


pub struct Frustum {
    pub origin: V3,
    pub dir: V3,
    pub up: V3,
}

impl Frustum {
    // XXX: A bunch of hardcoding
    pub fn ray(&self, screen_x: u32, screen_y: u32) -> Ray {
        let x = (fp(screen_x as i32) - fp(32)) / fp(16);
        let y = (fp(screen_y as i32) - fp(32)) / fp(32);

        let right = self.dir.cross(&self.up).normalized();

        let dir = (self.dir + self.up * -y + right * x).normalized();

        Ray {
            origin: self.origin,
            dir: dir,
        }
    }
}


#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: V3,
    pub dir: V3,
}

#[derive(Copy, Clone)]
pub struct Intersection {
    pub distance: FP,
    pub normal: V3,
}

pub trait Body {
    fn intersection(&self, ray: &Ray) -> Option<Intersection>;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: V3,
    pub radius: FP,
}

impl Body for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
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

        Some(Intersection {
            distance: p,
            normal: normal,
        })
    }
}

#[derive(Copy, Clone)]
pub struct Plane {
    pub normal: V3,
    pub offset: FP,
}

impl Body for Plane {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let a = self.normal.dot(&ray.dir);

        if a.abs() > FP(1) {
            let p0 = self.normal * self.offset;
            let d = (p0 - ray.origin).dot(&self.normal) / a;
            if d >= fp(0) {
                return Some(Intersection {
                    distance: d,
                    normal: self.normal,
                });
            }
        }

        None
    }
}
