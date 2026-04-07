mod scalar;
mod complex;
mod vector;
mod matrix;
mod extra;

use complex::Complex;
use vector::{angle_cos, cross_product, linear_combination, Vector};
use matrix::Matrix;
use extra::{lerp, projection};

fn main() {
    println!("========== ENTER THE MATRIX : TEST SUITE ==========\n");

    // ---------------------------------------------------------
    println!("--- [Ex 00] Vector & Matrix Add / Sub / Scl ---");
    
    // Tests Vecteurs
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
    println!("Vector Add:\n{}", u);
    let mut u_sub = Vector::from([2., 3.]);
    u_sub.sub(&v);
    println!("Vector Sub:\n{}", u_sub);
    u.scl(2.0);
    println!("Vector Scl x2:\n{}\n", u);

    // Tests Matrices
    let m1 = Matrix::from([[1., 2.], [3., 4.]]);
    let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
    
    println!("Matrix Shape : {:?}", m1.shape());
    
    let mut m_add = m1.clone();
    m_add.add(&m2);
    println!("Matrix Add :\n{}", m_add);

    let mut m_sub = m1.clone();
    m_sub.sub(&m2);
    println!("Matrix Sub :\n{}", m_sub);

    let mut m_scl = m1.clone();
    m_scl.scl(2.0);
    println!("Matrix Scl x2 :\n{}\n", m_scl);

    // ---------------------------------------------------------
    println!("--- [Ex 01] Linear Combination ---");
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    println!("Combinaison linéaire :\n{}\n", linear_combination(&[e1, e2, e3], &[10., -2., 0.5]));

    // ---------------------------------------------------------
    println!("--- [Ex 02] Lerp ---");
    let u_vec = Vector::from([2., 1.]);
    let v_vec = Vector::from([4., 2.]);
    println!("Lerp (t=0.3) :\n{}\n", lerp(u_vec, v_vec, 0.3));

    // ---------------------------------------------------------
    println!("--- [Ex 03] Dot Product ---");
    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("Dot u.v = {}\n", u.dot(&v));

    // ---------------------------------------------------------
    println!("--- [Ex 04] Norms ---");
    let mut u = Vector::from([-1., -2.]);
    println!("Norm 1: {}, Norm 2: {}, Norm Inf: {}\n", u.norm_1(), u.norm(), u.norm_inf());

    // ---------------------------------------------------------
    println!("--- [Ex 05] Angle Cosine ---");
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("Cos(u, v) = {}\n", angle_cos(&u, &v));

    // ---------------------------------------------------------
    println!("--- [Ex 06] Cross Product ---");
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("Cross u x v :\n{}\n", cross_product(&u, &v));

    // ---------------------------------------------------------
    println!("--- [Ex 07] Matrix Multiplication ---");
    
    // Matrice * Matrice
    let u = Matrix::from([[3., -5.], [6., 8.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("Matrice * Matrice =\n{}\n", u.mul_mat(&v));

    // Matrice * Vecteur
    let m3 = Matrix::from([[2., 0.], [0., 2.]]);
    let v_mult = Vector::from([4., 2.]);
    println!("Matrice * Vecteur =\n{}\n", m3.mul_vec(&v_mult));

    // ---------------------------------------------------------
    println!("--- [Ex 08] Trace ---");
    let mut u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    println!("Trace = {}\n", u.trace());

    // ---------------------------------------------------------
    println!("--- [Ex 09] Transpose ---");
    let mut v = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    println!("Transposée :\n{}\n", v.transpose());

    // ---------------------------------------------------------
    println!("--- [Ex 10] Row Echelon Form ---");
    let mut u = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
    ]);
    println!("RREF :\n{}\n", u.row_echelon());

    // ---------------------------------------------------------
    println!("--- [Ex 11] Determinant ---");
    let mut det_m = Matrix::from([
        [8., 5., -2.],
        [4., 7., 20.],
        [7., 6.,  1.],
    ]);
    println!("Det = {}\n", det_m.determinant());

    // ---------------------------------------------------------
    println!("--- [Ex 12] Matrix Inverse ---");
    let mut u = Matrix::from([
        [8., 5., -2.],
        [4., 7., 20.],
        [7., 6., 1.],
    ]);
    println!("Inverse :\n{}\n", u.inverse().unwrap());

    // ---------------------------------------------------------
    println!("--- [Ex 13] Rank ---");
    let mut u = Matrix::from([
        [ 1., 2., 0., 0.],
        [ 2., 4., 0., 0.],
        [-1., 2., 1., 1.],
    ]);
    println!("Rank = {}\n", u.rank());

    // ---------------------------------------------------------
    println!("--- [Ex 14] Projection Matrix ---");
    let proj = projection(std::f32::consts::PI / 2.0, 16.0 / 9.0, 0.1, 100.0);
    println!("{}\n", proj);

    // ---------------------------------------------------------
    println!("========== [Ex 15] TESTS NOMBRES COMPLEXES ==========\n");
    let c1 = Complex::new(1.0, 2.0); // 1 + 2i
    let c2 = Complex::new(3.0, -1.0); // 3 - 1i
    let c3 = Complex::new(0.0, 1.0); // i
    let c4 = Complex::new(2.0, 0.0); // 2

    println!("> Addition Vectorielle Complexe :");
    let mut v1 = Vector::from([c1, c2]);
    let v2 = Vector::from([c3, c4]);
    v1.add(&v2);
    println!("{}\n", v1);

    println!("> LERP Complexe (t = 0.5) :");
    let v_start = Vector::from([Complex::new(0.0, 0.0)]);
    let v_end = Vector::from([Complex::new(10.0, 10.0)]);
    println!("{}\n", lerp(v_start, v_end, 0.5));

    println!("> Inverse Matrice Complexe :");
    let mut c_mat = Matrix::from([
        [Complex::new(1.0, 0.0), Complex::new(0.0, 1.0)],
        [Complex::new(0.0, 1.0), Complex::new(1.0, 0.0)],
    ]);
    println!("{}\n", c_mat.inverse().unwrap());

    println!("TOUS LES TESTS SONT PASSÉS AVEC SUCCÈS ! 🎉");
}