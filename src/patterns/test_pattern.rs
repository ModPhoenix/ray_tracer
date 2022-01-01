use crate::{color::Color, matrix::Matrix, tuple::Tuple};

use super::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct TestPattern {
    transform: Matrix<4>,
}

impl TestPattern {
    pub fn new(transform: Matrix<4>) -> Self {
        Self { transform }
    }
}

impl Default for TestPattern {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(),
        }
    }
}

impl Pattern for TestPattern {
    fn get_transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self.clone()
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        Color::new(point.x, point.y, point.z)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        color::Color,
        matrix::Matrix,
        patterns::{test_pattern::TestPattern, Pattern},
        shapes::sphere::Sphere,
        tuple::Tuple,
    };

    #[test]
    fn the_default_pattern_transformation() {
        let pattern = TestPattern::default();

        assert_eq!(pattern.transform, Matrix::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let pattern =
            TestPattern::default().set_transform(Matrix::identity().translation(1., 2., 3.));

        assert_eq!(
            pattern.transform,
            Matrix::identity().translation(1., 2., 3.)
        );
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let shape = Sphere::default().set_transform(Matrix::identity().scaling(2., 2., 2.));
        let pattern = TestPattern::default();

        let c = pattern.pattern_at_shape(Rc::new(shape), Tuple::point(2., 3., 4.));

        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let shape = Sphere::default();
        let pattern = TestPattern::default().set_transform(Matrix::identity().scaling(2., 2., 2.));

        let c = pattern.pattern_at_shape(Rc::new(shape), Tuple::point(2., 3., 4.));

        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let shape = Sphere::default().set_transform(Matrix::identity().scaling(2., 2., 2.));
        let pattern =
            TestPattern::default().set_transform(Matrix::identity().translation(0.5, 1., 1.5));

        let c = pattern.pattern_at_shape(Rc::new(shape), Tuple::point(2.5, 3., 3.5));

        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
