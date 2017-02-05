use core::ops::{Add, Sub, Mul, Neg};
use fp::{fp, FP, EPSILON};

const I: V3 = v3(1, 0, 0);
const J: V3 = v3(0, 1, 0);
const K: V3 = v3(0, 0, 1);

#[derive(Copy, Clone)]
pub struct V3 {
    pub x: FP,
    pub y: FP,
    pub z: FP,
}

pub const fn v3(x: i32, y: i32, z: i32) -> V3 {
    V3::new(fp(x), fp(y), fp(z))
}

impl V3 {
    pub const fn new(x: FP, y: FP, z: FP) -> V3 {
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

    pub fn abs(&self) -> FP {
        self.dot(self).sqrt()
    }

    /// Normalized scalar field gradient.
    pub fn grad<F>(&self, f: F) -> V3 where F: Fn(V3) -> FP {
        V3::new(
            f(*self + I * EPSILON) - f(*self - I * EPSILON),
            f(*self + J * EPSILON) - f(*self - J * EPSILON),
            f(*self + K * EPSILON) - f(*self - K * EPSILON))
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

impl Neg for V3 {
    type Output = V3;

    fn neg(self) -> V3 {
        V3::new(-self.x, -self.y, -self.z)
    }
}
