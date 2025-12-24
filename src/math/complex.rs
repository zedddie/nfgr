use std::ops::{Add, Div, Mul, Neg, Sub};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub rl: f64,
    pub im: f64,
}
impl Complex {
    pub fn new(rl: f64, im: f64) -> Self {
        Self { rl, im }
    }
    pub fn sqr(self) -> Self {
        Self {
            rl: self.rl * self.rl - self.im * self.im,
            im: 2_f64 * self.rl * self.im,
        }
    }
    pub fn n_sqr(self) -> f64 {
        self.rl * self.rl + self.im + self.im
    }
}
#[derive(Clone, Copy, Debug)]
pub struct DComplex {
    pub val: Complex,
    pub der: Complex,
}
impl DComplex {
    pub fn var(z: Complex) -> Self {
        Self {
            val: z,
            der: Complex::new(1.0, 0.0),
        }
    }
    pub fn cst(z: Complex) -> Self {
        Self {
            val: z,
            der: Complex::new(0.0, 0.0),
        }
    }

    pub fn powi(self, n: i32) -> Self {
        let mut result = self;
        for _ in 1..n {
            result = result * self;
        }
        result
    }
}
impl Add for DComplex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            val: self.val + rhs.val,
            der: self.der + rhs.der,
        }
    }
}
impl Sub for DComplex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            val: self.val - rhs.val,
            der: self.der - rhs.der,
        }
    }
}
impl Mul for DComplex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            val: self.val * rhs.val,
            der: (self.der * rhs.val) + (self.val * rhs.der),
        }
    }
}
impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            rl: self.rl + rhs.rl,
            im: self.im + rhs.im,
        }
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            rl: self.rl - rhs.rl,
            im: self.im - rhs.im,
        }
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            rl: self.rl * rhs.rl - self.im * rhs.im,
            im: self.rl * rhs.im + self.im * rhs.rl,
        }
    }
}
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let den = rhs.rl * rhs.rl + rhs.im * rhs.im;
        if den.abs() < f64::EPSILON {
            return Complex::new(f64::NAN, f64::NAN);
        }
        let r_rl = (self.rl * rhs.rl + self.im * rhs.im) / den;
        let r_im = (self.im * rhs.rl - self.rl * rhs.im) / den;

        Complex::new(r_rl, r_im)
    }
}
