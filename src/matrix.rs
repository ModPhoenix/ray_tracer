use std::ops::Index;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const D: usize> {
    data: [[f64; D]; D],
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

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix::from([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
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
}
