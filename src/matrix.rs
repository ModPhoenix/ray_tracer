use std::ops::{Index, IndexMut, Mul};

use crate::{tuple::Tuple, utils::equal::equal};

#[derive(Debug, Clone)]
pub struct Matrix<const D: usize> {
    data: [[f64; D]; D],
}

impl<const D: usize> Matrix<D> {
    fn new() -> Self {
        Self::from([[0.; D]; D])
    }

    pub fn identity() -> Self {
        let mut result = Self::new();

        for n in 0..D {
            result[n][n] = 1.;
        }

        result
    }

    pub fn transpose(&self) -> Self {
        let mut result = Self::new();

        for row in 0..D {
            for col in 0..D {
                result[row][col] = self[col][row];
            }
        }

        result
    }

    fn get_submatrix<const L: usize>(&self, remove_row: usize, remove_col: usize) -> Matrix<L> {
        let mut result = Matrix::<L>::new();

        for row in 0..D {
            for col in 0..D {
                if row < remove_row {
                    if col < remove_col {
                        result[row][col] = self[row][col];
                    } else if col > remove_col {
                        result[row][col - 1] = self[row][col];
                    }
                } else if row > remove_row {
                    if col < remove_col {
                        result[row - 1][col] = self[row][col];
                    } else if col > remove_col {
                        result[row - 1][col - 1] = self[row][col];
                    }
                }
            }
        }

        result
    }
}

impl Matrix<4> {
    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix<3> {
        self.get_submatrix(remove_row, remove_col)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let result = self.minor(row, col);

        if (row + col) % 2 == 0 {
            result
        } else {
            -result
        }
    }

    fn determinant(&self) -> f64 {
        let mut det = 0.;

        for col in 0..4 {
            det += self[0][col] * self.cofactor(0, col);
        }

        det
    }

    fn is_invertible(&self) -> bool {
        self.determinant() != 0.
    }

    fn inverse(&self) -> Self {
        if !self.is_invertible() {
            panic!()
        }

        let mut result = Self::new();
        let determinant = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                let cofactor = self.cofactor(row, col);

                result[col][row] = cofactor / determinant;
            }
        }

        result
    }

    fn translation(mut self, x: f64, y: f64, z: f64) -> Self {
        self[0][3] = x;
        self[1][3] = y;
        self[2][3] = z;

        self
    }

    fn scaling(mut self, x: f64, y: f64, z: f64) -> Self {
        self[0][0] = x;
        self[1][1] = y;
        self[2][2] = z;

        self
    }

    fn rotation_x(mut self, radians: f64) -> Self {
        self[1][1] = radians.cos();
        self[1][2] = -radians.sin();
        self[2][1] = radians.sin();
        self[2][2] = radians.cos();

        self
    }

    fn rotation_y(mut self, radians: f64) -> Self {
        self[0][0] = radians.cos();
        self[0][2] = radians.sin();
        self[2][0] = -radians.sin();
        self[2][2] = radians.cos();

        self
    }
}

impl Matrix<3> {
    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix<2> {
        self.get_submatrix(remove_row, remove_col)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let result = self.minor(row, col);

        if (row + col) % 2 == 0 {
            result
        } else {
            -result
        }
    }

    fn determinant(&self) -> f64 {
        let mut det = 0.;

        for col in 0..3 {
            det += self[0][col] * self.cofactor(0, col);
        }

        det
    }
}

impl Matrix<2> {
    fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl<const D: usize> From<[[f64; D]; D]> for Matrix<D> {
    fn from(data: [[f64; D]; D]) -> Self {
        Matrix { data }
    }
}

impl<const D: usize> Index<usize> for Matrix<D> {
    type Output = [f64; D];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const D: usize> IndexMut<usize> for Matrix<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const D: usize> PartialEq for Matrix<D> {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..D {
            for col in 0..D {
                if !equal(self[row][col], other[row][col]) {
                    return false;
                }
            }
        }

        return true;
    }
}

impl<const D: usize> Mul for Matrix<D> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Self::new();

        for row in 0..D {
            for col in 0..D {
                for n in 0..D {
                    result[row][col] = result[row][col] + self[row][n] * rhs[n][col];
                }
            }
        }

        result
    }
}

impl Mul<Tuple> for Matrix<4> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3] * rhs.w,
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3] * rhs.w,
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3] * rhs.w,
            self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3] * rhs.w,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{matrix::Matrix, tuple::Tuple};

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        #[rustfmt::skip]
        let m = Matrix::from([
            [  1.,   2.,   3.,   4.],
            [ 5.5,  6.5,  7.5,  8.5],
            [  9.,  10.,  11.,  12.],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[0][0], 1.);
        assert_eq!(m[0][3], 4.);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        #[rustfmt::skip]
        let m = Matrix::from([
            [-3.,  5.],
            [ 1., -2.],
        ]);

        assert_eq!(m[0][0], -3.);
        assert_eq!(m[0][1], 5.);
        assert_eq!(m[1][0], 1.);
        assert_eq!(m[1][1], -2.);
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        #[rustfmt::skip]
        let m = Matrix::from([
            [-3.,  5.,  0.],
            [ 1., -2., -7.],
            [ 0.,  1.,  1.],
        ]);

        assert_eq!(m[0][0], -3.);
        assert_eq!(m[1][1], -2.);
        assert_eq!(m[2][2], 1.);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix::from([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        let b = Matrix::from([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        assert!(a == b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix::from([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        let b = Matrix::from([
            [2., 3., 4., 5.],
            [6., 7., 8., 9.],
            [8., 7., 6., 5.],
            [4., 3., 2., 1.],
        ]);

        assert!(a != b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix::from([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        let b = Matrix::from([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);

        assert_eq!(
            a * b,
            Matrix::from([
                [20., 22., 50., 48.],
                [44., 54., 114., 108.],
                [40., 58., 110., 102.],
                [16., 26., 46., 42.],
            ]),
        );
    }

    #[test]
    fn matrix_multiplied_by_a_tuple() {
        let m = Matrix::from([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        ]);

        let t = Tuple::new(1., 2., 3., 1.);

        assert_eq!(m * t, Tuple::new(18., 24., 33., 1.));
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let m = Matrix::from([
            [0., 1., 2., 4.],
            [1., 2., 4., 8.],
            [2., 4., 8., 16.],
            [4., 8., 16., 32.],
        ]);

        assert_eq!(
            m * Matrix::<4>::identity(),
            Matrix::from([
                [0., 1., 2., 4.],
                [1., 2., 4., 8.],
                [2., 4., 8., 16.],
                [4., 8., 16., 32.],
            ])
        );
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let t = Tuple::new(1., 2., 3., 4.);

        assert_eq!(Matrix::<4>::identity() * t, Tuple::new(1., 2., 3., 4.));
    }

    #[test]
    fn transposing_a_matrix() {
        let m = Matrix::from([
            [0., 9., 3., 0.],
            [9., 8., 0., 8.],
            [1., 8., 5., 3.],
            [0., 0., 5., 8.],
        ]);

        assert_eq!(
            m.transpose(),
            Matrix::from([
                [0., 9., 1., 0.],
                [9., 8., 8., 0.],
                [3., 0., 5., 5.],
                [0., 8., 3., 8.],
            ])
        );
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let m = Matrix::<4>::identity();

        assert_eq!(m.transpose(), Matrix::<4>::identity());
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        #[rustfmt::skip]
        let m = Matrix::from([
            [ 1., 5.],
            [-3., 2.],
        ]);

        assert_eq!(m.determinant(), 17.);
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        #[rustfmt::skip]
        let m = Matrix::from([
            [ 1., 5.,  0.],
            [-3., 2.,  7.],
            [ 0., 6., -3.],
        ]);

        #[rustfmt::skip]
        assert_eq!(m.submatrix(0, 2), Matrix::from([
            [-3., 2.],
            [ 0., 6.]
        ]));
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        #[rustfmt::skip]
        let m = Matrix::from([
            [-6., 1.,  1., 6.],
            [-8., 5.,  8., 6.],
            [-1., 0.,  8., 2.],
            [-7., 1., -1., 1.],
        ]);

        #[rustfmt::skip]
        assert_eq!(m.submatrix(2, 1), Matrix::from([
            [-6.,  1., 6.],
            [-8.,  8., 6.],
            [-7., -1., 1.]
        ]));
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        #[rustfmt::skip]
        let a = Matrix::from([
            [3.,  5.,  0.],
            [2., -1., -7.],
            [6., -1.,  5.],
        ]);

        let b = a.submatrix(1, 0);

        assert_eq!(b.determinant(), 25.);
        assert_eq!(a.minor(1, 0), 25.);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        #[rustfmt::skip]
        let a = Matrix::from([
            [3.,  5.,  0.],
            [2., -1., -7.],
            [6., -1.,  5.],
        ]);

        assert_eq!(a.minor(0, 0), -12.);
        assert_eq!(a.cofactor(0, 0), -12.);
        assert_eq!(a.minor(1, 0), 25.);
        assert_eq!(a.cofactor(1, 0), -25.);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        #[rustfmt::skip]
        let a = Matrix::from([
            [ 1., 2.,  6.],
            [-5., 8., -4.],
            [ 2., 6.,  4.],
        ]);

        assert_eq!(a.cofactor(0, 0), 56.);
        assert_eq!(a.cofactor(0, 1), 12.);
        assert_eq!(a.cofactor(0, 2), -46.);
        assert_eq!(a.determinant(), -196.);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        #[rustfmt::skip]
        let a = Matrix::from([
            [-2., -8.,  3.,  5.],
            [-3.,  1.,  7.,  3.],
            [ 1.,  2., -9.,  6.],
            [-6.,  7.,  7., -9.],
        ]);

        assert_eq!(a.cofactor(0, 0), 690.);
        assert_eq!(a.cofactor(0, 1), 447.);
        assert_eq!(a.cofactor(0, 2), 210.);
        assert_eq!(a.cofactor(0, 3), 51.);
        assert_eq!(a.determinant(), -4071.);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        #[rustfmt::skip]
        let a = Matrix::from([
            [6.,  4., 4.,  4.],
            [5.,  5., 7.,  6.],
            [4., -9., 3., -7.],
            [9.,  1., 7., -6.],
        ]);

        assert_eq!(a.determinant(), -2120.);
        assert!(a.is_invertible());
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        #[rustfmt::skip]
        let a = Matrix::from([
            [-4.,  2., -2., -3.],
            [ 9.,  6.,  2.,  6.],
            [ 0., -5.,  1., -5.],
            [ 0.,  0.,  0.,  0.],
        ]);

        assert_eq!(a.determinant(), 0.);
        assert!(!a.is_invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        #[rustfmt::skip]
        let a = Matrix::from([
            [-5.,  2.,  6., -8.],
            [ 1., -5.,  1.,  8.],
            [ 7.,  7., -6., -7.],
            [ 1., -3.,  7.,  4.],
        ]);
        let b = a.inverse();

        assert_eq!(a.determinant(), 532.);
        assert_eq!(a.cofactor(2, 3), -160.);
        assert_eq!(b[3][2], -160. / 532.);
        assert_eq!(a.cofactor(3, 2), 105.);
        assert_eq!(b[2][3], 105. / 532.);
        #[rustfmt::skip]
        assert_eq!(b, Matrix::from([
            [ 0.21805,  0.45113,  0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361,  0.52068],
            [-0.07895, -0.22368, -0.05263,  0.19737],
            [-0.52256, -0.81391, -0.30075,  0.30639],
        ]));
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        #[rustfmt::skip]
        let a = Matrix::from([
            [ 8., -5.,  9.,  2.],
            [ 7.,  5.,  6.,  1.],
            [-6.,  0.,  9.,  6.],
            [-3.,  0., -9., -4.],
        ]);
        let b = a.inverse();

        #[rustfmt::skip]
        assert_eq!(b, Matrix::from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692,  0.12308,  0.02564,  0.03077],
            [ 0.35897,  0.35897,  0.43590,  0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]));
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        #[rustfmt::skip]
        let a = Matrix::from([
            [ 9.,  3.,  0.,  9.],
            [-5., -2., -6., -3.],
            [-4.,  9.,  6.,  4.],
            [-7.,  6.,  6.,  2.],
        ]);
        let b = a.inverse();

        #[rustfmt::skip]
        assert_eq!(b, Matrix::from([
            [-0.04074, -0.07778,  0.14444, -0.22222],
            [-0.07778,  0.03333,  0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926,  0.12963],
            [ 0.17778,  0.06667, -0.26667,  0.33333],
        ]));
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        #[rustfmt::skip]
        let a = Matrix::from([
            [ 3., -9.,  7.,  3.],
            [ 3., -8.,  2., -9.],
            [-4.,  4.,  4.,  1.],
            [-6.,  5., -1.,  1.],
        ]);

        #[rustfmt::skip]
        let b = Matrix::from([
            [8.,  2., 2., 2.],
            [3., -1., 7., 0.],
            [7.,  0., 5., 4.],
            [6., -2., 0., 5.],
        ]);

        let c = a.clone() * b.clone();

        assert_eq!(c * b.inverse(), a);
    }

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix::identity().translation(5., -3., 2.);
        let p = Tuple::point(-3., 4., 5.);

        assert_eq!(transform * p, Tuple::point(2., 1., 7.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix::identity().translation(5., -3., 2.);
        let inv = transform.inverse();
        let p = Tuple::point(-3., 4., 5.);

        assert_eq!(inv * p, Tuple::point(-8., 7., 3.));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::identity().translation(5., -3., 2.);
        let v = Tuple::vector(-3., 4., 5.);

        assert_eq!(transform * v.clone(), v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Matrix::identity().scaling(2., 3., 4.);
        let p = Tuple::point(-4., 6., 8.);

        assert_eq!(transform * p, Tuple::point(-8., 18., 32.));
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Matrix::identity().scaling(2., 3., 4.);
        let v = Tuple::vector(-4., 6., 8.);

        assert_eq!(transform * v, Tuple::vector(-8., 18., 32.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let inv = Matrix::identity().scaling(2., 3., 4.).inverse();
        let v = Tuple::vector(-4., 6., 8.);

        assert_eq!(inv * v, Tuple::vector(-2., 2., 2.));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix::identity().scaling(-1., 1., 1.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(transform * p, Tuple::point(-2., 3., 4.));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0., 1., 0.);

        let half_quarter = Matrix::identity().rotation_x(PI / 4.);
        let full_quarter = Matrix::identity().rotation_x(PI / 2.);

        assert_eq!(
            half_quarter * p.clone(),
            Tuple::point(0., 2.0_f64.sqrt() / 2., 2.0_f64.sqrt() / 2.)
        );
        assert_eq!(full_quarter * p, Tuple::point(0., 0., 1.));
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0., 1., 0.);

        let half_quarter = Matrix::identity().rotation_x(PI / 4.);
        let inv = half_quarter.inverse();

        assert_eq!(
            inv * p,
            Tuple::point(0., 2.0_f64.sqrt() / 2., -(2.0_f64.sqrt() / 2.))
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0., 0., 1.);

        let half_quarter = Matrix::identity().rotation_y(PI / 4.);
        let full_quarter = Matrix::identity().rotation_y(PI / 2.);

        assert_eq!(
            half_quarter * p.clone(),
            Tuple::point(2.0_f64.sqrt() / 2., 0., 2.0_f64.sqrt() / 2.)
        );
        assert_eq!(full_quarter * p, Tuple::point(1., 0., 0.));
    }
}
