use std::fmt::Display;
use crate::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<K> {
    pub data: Vec<K>,
}

impl<K: Scalar> Vector<K> {
    pub fn size(&self) -> usize { self.data.len() }

    pub fn add(&mut self, v: &Vector<K>) {
        assert_eq!(self.size(), v.size(), "Size mismatch");
        for i in 0..self.size() { self.data[i] = self.data[i] + v.data[i]; }
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        assert_eq!(self.size(), v.size(), "Size mismatch");
        for i in 0..self.size() { self.data[i] = self.data[i] - v.data[i]; }
    }

    pub fn scl(&mut self, a: K) {
        for i in 0..self.size() { self.data[i] = self.data[i] * a; }
    }

    pub fn dot(&self, v: &Vector<K>) -> K {
        assert_eq!(self.size(), v.size(), "Size mismatch");
        let mut result = K::zero();
        for i in 0..self.size() {
            result = K::fma(self.data[i].conjugate(), v.data[i], result);
        }
        result
    }

    pub fn norm_1(&mut self) -> f32 {
        let mut sum = 0.0;
        for i in 0..self.size() { sum += self.data[i].abs(); }
        sum
    }

    pub fn norm(&mut self) -> f32 {
        self.dot(self).abs().powf(0.5)
    }

    pub fn norm_inf(&mut self) -> f32 {
        let mut max_val = 0.0_f32;
        for i in 0..self.size() { max_val = max_val.max(self.data[i].abs()); }
        max_val
    }
}

// Fonctions libres (Ex 01, 05, 06)
pub fn linear_combination<K: Scalar>(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
    assert_eq!(u.len(), coefs.len(), "Size mismatch");
    let vec_size = u[0].size();
    let mut result_data = vec![K::zero(); vec_size];
    for i in 0..vec_size {
        for j in 0..u.len() {
            result_data[i] = K::fma(coefs[j], u[j].data[i], result_data[i]);
        }
    }
    Vector { data: result_data }
}

pub fn angle_cos<K: Scalar>(u: &Vector<K>, v: &Vector<K>) -> f32 {
    let mut u_clone = u.clone();
    let mut v_clone = v.clone();
    let norm_u = u_clone.norm();
    let norm_v = v_clone.norm();
    assert!(norm_u > 0.0 && norm_v > 0.0, "Zero vector");
    u.dot(v).real_part() / (norm_u * norm_v)
}

pub fn cross_product<K: Scalar>(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
    assert_eq!(u.size(), 3, "Cross product needs 3D");
    assert_eq!(v.size(), 3, "Cross product needs 3D");
    let w_x = (u.data[1] * v.data[2]) - (u.data[2] * v.data[1]);
    let w_y = (u.data[2] * v.data[0]) - (u.data[0] * v.data[2]);
    let w_z = (u.data[0] * v.data[1]) - (u.data[1] * v.data[0]);
    Vector { data: vec![w_x, w_y, w_z] }
}

// Utilitaires
impl<K: Copy, const N: usize> From<[K; N]> for Vector<K> {
    fn from(arr: [K; N]) -> Self { Vector { data: arr.to_vec() } }
}

impl<K: Display> Display for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, val) in self.data.iter().enumerate() {
            write!(f, "[{}]", val)?;
            if i < self.data.len() - 1 { writeln!(f)?; }
        }
        Ok(())
    }
}
