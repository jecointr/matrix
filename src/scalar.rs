use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar:
    Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn fma(a: Self, b: Self, c: Self) -> Self;
    fn conjugate(self) -> Self;
    fn abs(self) -> f32;
    fn real_part(self) -> f32;
    fn is_nearly_zero(&self) -> bool;
}

impl Scalar for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn fma(a: Self, b: Self, c: Self) -> Self { a.mul_add(b, c) }
    fn conjugate(self) -> Self { self }
    fn abs(self) -> f32 { f32::abs(self) }
    fn real_part(self) -> f32 { self }
    fn is_nearly_zero(&self) -> bool { self.abs() < 1e-5 }
}
