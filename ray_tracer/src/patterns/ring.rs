use crate::{color::Color, matrix::Matrix, tuple::Tuple};

use super::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct Ring {
    a: Color,
    b: Color,
    transform: Matrix<4>,
}

impl Ring {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Matrix::identity(),
        }
    }
}

impl Pattern for Ring {
    fn get_transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self.clone()
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        if (point.x.powf(2.) + point.z.powf(2.)).sqrt().floor() % 2.0 == 0.0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, patterns::ring::Ring, tuple::Tuple};

    use super::Pattern;

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Ring::new(Color::new_white(), Color::new_black());

        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(1., 0., 0.)),
            Color::new_black()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 1.)),
            Color::new_black()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.708, 0., 0.708)),
            Color::new_black()
        );
    }
}
