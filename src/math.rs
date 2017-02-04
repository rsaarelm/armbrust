use core::ops::{Add, Sub, Mul, Div, Neg};

const DECIMAL_BITS: usize = 8;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct FP(pub i32);

pub fn fp(a: i32) -> FP {
    FP(a << DECIMAL_BITS)
}

impl FP {
    pub fn sqrt(self) -> FP {
        // Crappy Newton's method.
        let mut x = self / fp(2);
        for _ in 0..10 {
            x = (x + self / x) / fp(2);
        }
        x
    }

    pub fn to_i32(self) -> i32 {
        self.0 >> DECIMAL_BITS
    }

    pub fn abs(self) -> FP {
        if self.0 < 0 {
            FP(-self.0)
        } else {
            self
        }
    }
}

impl Add for FP {
    type Output = FP;
    fn add(self, other: FP) -> FP {
        FP(self.0 + other.0)
    }
}

impl Sub for FP {
    type Output = FP;
    fn sub(self, other: FP) -> FP {
        FP(self.0 - other.0)
    }
}

impl Mul<FP> for FP {
    type Output = FP;
    fn mul(self, other: FP) -> FP {
        FP(((self.0 as i64 * other.0 as i64) >> DECIMAL_BITS) as i32)
    }
}

impl Div<FP> for FP {
    type Output = FP;
    fn div(self, other: FP) -> FP {
        FP((((self.0 as i64) << DECIMAL_BITS) / other.0 as i64) as i32)
    }
}

impl Neg for FP {
    type Output = FP;
    fn neg(self) -> FP {
        FP(-self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::fp;

    #[test]
    fn test_fp() {
        assert!(fp(1) + fp(1) == fp(2));

        assert!(fp(10000).sqrt().to_i32() == 100);
    }
}
