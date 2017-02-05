use fp::{fp, FP};
use v::{v3, V3};

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
