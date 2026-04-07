use crate::scalar::Scalar;
use crate::vector::Vector;
use crate::matrix::Matrix;

pub trait Lerp {
    fn lerp(self, v: Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(self, v: Self, t: f32) -> Self { f32::fma(t, v - self, self) }
}

impl<K: Scalar + From<f32>> Lerp for Vector<K> {
    fn lerp(self, v: Self, t: f32) -> Self {
        let mut result = Vec::with_capacity(self.data.len());
        let t_k = K::from(t);
        for i in 0..self.data.len() {
            result.push(K::fma(t_k, v.data[i] - self.data[i], self.data[i]));
        }
        Vector { data: result }
    }
}

impl<K: Scalar + From<f32>> Lerp for Matrix<K> {
    fn lerp(self, m: Self, t: f32) -> Self {
        let mut result = Vec::with_capacity(self.data.len());
        let t_k = K::from(t);
        for i in 0..self.data.len() {
            result.push(K::fma(t_k, m.data[i] - self.data[i], self.data[i]));
        }
        Matrix { data: result, rows: self.rows, cols: self.cols }
    }
}

pub fn lerp<V: Lerp>(u: V, v: V, t: f32) -> V { u.lerp(v, t) }

pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    let f = 1.0 / (fov / 2.0).tan();
    let z_range = far - near;
    let a = -far / z_range;
    let b = -(far * near) / z_range;

    let data = vec![
        f / ratio, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, a, -1.0,
        0.0, 0.0, b, 0.0,
    ];

    Matrix { data, rows: 4, cols: 4 }
}
