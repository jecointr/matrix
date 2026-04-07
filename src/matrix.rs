use std::fmt::Display;
use crate::scalar::Scalar;
use crate::vector::Vector;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<K> {
    pub data: Vec<K>,
    pub rows: usize,
    pub cols: usize,
}

impl<K: Scalar> Matrix<K> {
    pub fn get(&self, r: usize, c: usize) -> K { self.data[c * self.rows + r] }
    pub fn set(&mut self, r: usize, c: usize, val: K) { self.data[c * self.rows + r] = val; }
    pub fn shape(&self) -> (usize, usize) { (self.rows, self.cols) }
    pub fn is_square(&self) -> bool { self.rows == self.cols }

    pub fn swap_rows(&mut self, r1: usize, r2: usize) {
        if r1 == r2 { return; }
        for c in 0..self.cols {
            let tmp = self.get(r1, c);
            self.set(r1, c, self.get(r2, c));
            self.set(r2, c, tmp);
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut data = vec![K::zero(); size * size];
        for i in 0..size { data[i * size + i] = K::one(); }
        Matrix { data, rows: size, cols: size }
    }

    pub fn add(&mut self, m: &Matrix<K>) {
        assert_eq!(self.shape(), m.shape(), "Shape mismatch");
        for i in 0..self.data.len() { self.data[i] = self.data[i] + m.data[i]; }
    }

    pub fn sub(&mut self, m: &Matrix<K>) {
        assert_eq!(self.shape(), m.shape(), "Shape mismatch");
        for i in 0..self.data.len() { self.data[i] = self.data[i] - m.data[i]; }
    }

    pub fn scl(&mut self, a: K) {
        for i in 0..self.data.len() { self.data[i] = self.data[i] * a; }
    }

    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        assert_eq!(self.cols, vec.data.len(), "Size mismatch");
        let mut res = vec![K::zero(); self.rows];
        for r in 0..self.rows {
            for c in 0..self.cols {
                res[r] = K::fma(self.get(r, c), vec.data[c], res[r]);
            }
        }
        Vector { data: res }
    }

    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        assert_eq!(self.cols, mat.rows, "Size mismatch");
        let m = self.rows;
        let p = mat.cols;
        let mut result = Matrix { data: vec![K::zero(); m * p], rows: m, cols: p };
        for r in 0..m {
            for c in 0..p {
                let mut sum = K::zero();
                for k in 0..self.cols {
                    sum = K::fma(self.get(r, k), mat.get(k, c), sum);
                }
                result.data[c * m + r] = sum;
            }
        }
        result
    }

    pub fn trace(&mut self) -> K {
        assert!(self.is_square(), "Must be square");
        let mut sum = K::zero();
        for i in 0..self.rows { sum = sum + self.get(i, i); }
        sum
    }

    pub fn transpose(&mut self) -> Matrix<K> {
        let mut new_data = Vec::with_capacity(self.rows * self.cols);
        for c in 0..self.rows {
            for r in 0..self.cols {
                new_data.push(self.get(c, r).conjugate());
            }
        }
        Matrix { data: new_data, rows: self.cols, cols: self.rows }
    }

    pub fn row_echelon(&mut self) -> Matrix<K> {
        let mut res = self.clone();
        let mut r = 0;
        let mut c = 0;
        while r < res.rows && c < res.cols {
            let mut pivot_row = r;
            let mut max_val = res.get(r, c).abs();
            for i in (r + 1)..res.rows {
                let val = res.get(i, c).abs();
                if val > max_val {
                    max_val = val;
                    pivot_row = i;
                }
            }
            if res.get(pivot_row, c).is_nearly_zero() {
                c += 1;
                continue;
            }
            res.swap_rows(r, pivot_row);
            let pivot_val = res.get(r, c);
            for j in 0..res.cols { res.set(r, j, res.get(r, j) / pivot_val); }
            for i in 0..res.rows {
                if i != r {
                    let factor = res.get(i, c);
                    if !factor.is_nearly_zero() {
                        for j in 0..res.cols {
                            let sub_val = factor * res.get(r, j);
                            res.set(i, j, res.get(i, j) - sub_val);
                        }
                    }
                }
            }
            r += 1;
            c += 1;
        }
        res
    }

    fn submatrix(&self, exclude_row: usize, exclude_col: usize) -> Matrix<K> {
        let mut new_data = Vec::with_capacity((self.rows - 1) * (self.cols - 1));
        for c in 0..self.cols {
            if c == exclude_col { continue; }
            for r in 0..self.rows {
                if r == exclude_row { continue; }
                new_data.push(self.get(r, c));
            }
        }
        Matrix { data: new_data, rows: self.rows - 1, cols: self.cols - 1 }
    }

    pub fn determinant(&mut self) -> K {
        assert!(self.is_square(), "Must be square");
        let n = self.rows;
        if n == 1 { return self.get(0, 0); }
        if n == 2 {
            let a = self.get(0, 0); let b = self.get(0, 1);
            let c = self.get(1, 0); let d = self.get(1, 1);
            return K::fma(a, d, K::zero() - (b * c));
        }
        let mut det = K::zero();
        let mut sign_is_positive = true;
        for c in 0..n {
            let mut sub = self.submatrix(0, c);
            let sub_det = sub.determinant();
            let factor = self.get(0, c);
            if sign_is_positive {
                det = K::fma(factor, sub_det, det);
            } else {
                det = K::fma(K::zero() - factor, sub_det, det);
            }
            sign_is_positive = !sign_is_positive;
        }
        det
    }

    pub fn inverse(&mut self) -> Result<Matrix<K>, &'static str> {
        assert!(self.is_square(), "Must be square");
        let n = self.rows;
        let mut a = self.clone();
        let mut inv = Matrix::identity(n);

        for i in 0..n {
            let mut pivot_row = i;
            let mut max_val = a.get(i, i).abs();
            for r in (i + 1)..n {
                let val = a.get(r, i).abs();
                if val > max_val { max_val = val; pivot_row = r; }
            }
            if a.get(pivot_row, i).is_nearly_zero() {
                return Err("Matrix is singular");
            }
            a.swap_rows(i, pivot_row);
            inv.swap_rows(i, pivot_row);

            let pivot_val = a.get(i, i);
            for c in 0..n {
                a.set(i, c, a.get(i, c) / pivot_val);
                inv.set(i, c, inv.get(i, c) / pivot_val);
            }
            for r in 0..n {
                if r != i {
                    let factor = a.get(r, i);
                    if !factor.is_nearly_zero() {
                        let neg_factor = K::zero() - factor;
                        for c in 0..n {
                            let new_a = K::fma(neg_factor, a.get(i, c), a.get(r, c));
                            a.set(r, c, new_a);
                            let new_inv = K::fma(neg_factor, inv.get(i, c), inv.get(r, c));
                            inv.set(r, c, new_inv);
                        }
                    }
                }
            }
        }
        Ok(inv)
    }

    pub fn rank(&mut self) -> usize {
        let rref = self.row_echelon();
        let mut rank = 0;
        for r in 0..rref.rows {
            let mut is_non_zero = false;
            for c in 0..rref.cols {
                if !rref.get(r, c).is_nearly_zero() { is_non_zero = true; break; }
            }
            if is_non_zero { rank += 1; }
        }
        rank
    }
}

// Utilitaires
impl<K: Copy + Default, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(arr: [[K; C]; R]) -> Self {
        let mut data = Vec::with_capacity(R * C);
        for c in 0..C {
            for r in 0..R { data.push(arr[r][c]); }
        }
        Matrix { data, rows: R, cols: C }
    }
}

impl<K: Display + Scalar> Display for Matrix<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows {
            write!(f, "[")?;
            for c in 0..self.cols {
                write!(f, "{}", self.get(r, c))?; 
                if c < self.cols - 1 { write!(f, " , ")?; }
            }
            write!(f, "]")?;
            if r < self.rows - 1 { writeln!(f)?; }
        }
        Ok(())
    }
}
