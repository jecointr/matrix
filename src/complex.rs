use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use crate::scalar::Scalar;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Complex { re, im }
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self { Complex::new(self.re + other.re, self.im + other.im) }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self { Complex::new(self.re - other.re, self.im - other.im) }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let den = other.re * other.re + other.im * other.im;
        Complex::new(
            (self.re * other.re + self.im * other.im) / den,
            (self.im * other.re - self.re * other.im) / den,
        )
    }
}

// Convertir un f32 en complexe (requis pour le Lerp - Ex 02)
impl From<f32> for Complex {
    fn from(re: f32) -> Self {
        Complex::new(re, 0.0)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{:.2} + {:.2}i", self.re, self.im)
        } else {
            write!(f, "{:.2} - {:.2}i", self.re, self.im.abs())
        }
    }
}

impl Scalar for Complex {
    fn zero() -> Self { Complex::new(0.0, 0.0) }
    fn one() -> Self { Complex::new(1.0, 0.0) }
    fn fma(a: Self, b: Self, c: Self) -> Self { (a * b) + c }
    fn conjugate(self) -> Self { Complex::new(self.re, -self.im) }
    fn abs(self) -> f32 { (self.re * self.re + self.im * self.im).sqrt() }
    fn real_part(self) -> f32 { self.re }
    fn is_nearly_zero(&self) -> bool { self.abs() < 1e-5 }
}
