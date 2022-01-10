use crate::{color::Color, matrix::Matrix, tuple::Tuple};

use super::Pattern;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Checkers {
    a: Color,
    b: Color,
    transform: Matrix<4>,
}

impl Checkers {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Matrix::identity(),
        }
    }
}

impl Pattern for Checkers {
    fn get_transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self.clone()
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, patterns::checkers::Checkers, tuple::Tuple};

    use super::Pattern;

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Checkers::new(Color::new_white(), Color::new_black());

        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.99, 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(1.01, 0., 0.)),
            Color::new_black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Checkers::new(Color::new_white(), Color::new_black());

        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0.99, 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 1.01, 0.)),
            Color::new_black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Checkers::new(Color::new_white(), Color::new_black());

        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 0.99,)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 1.01)),
            Color::new_black()
        );
    }
}
