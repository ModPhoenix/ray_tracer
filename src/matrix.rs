use std::ops::{Index, IndexMut, Mul};

use crate::tuple::Tuple;

#[derive(Debug, Clone, PartialEq)]
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
}

impl Matrix<3> {
    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix<2> {
        self.get_submatrix(remove_row, remove_col)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.get_submatrix(row, col).determinant()
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
}
