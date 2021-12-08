use crate::{color::Color, matrix::Matrix, tuple::Tuple};

use super::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct Gradient {
    a: Color,
    b: Color,
    transform: Matrix<4>,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Matrix::identity(),
        }
    }
}

impl Pattern for Gradient {
    fn get_transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self.clone()
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        let distance = self.b.clone() - self.a.clone();
        let fraction = point.x.fract();

        self.a.clone() + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, patterns::gradient::Gradient, tuple::Tuple};

    use super::Pattern;

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Gradient::new(Color::new_white(), Color::new_black());

        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.25, 0., 0.)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.5, 0., 0.)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.75, 0., 0.)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
