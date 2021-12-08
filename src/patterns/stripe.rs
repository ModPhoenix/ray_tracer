use crate::{color::Color, matrix::Matrix, tuple::Tuple};

use super::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct Stripe {
    a: Color,
    b: Color,
    transform: Matrix<4>,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Matrix::identity(),
        }
    }
}

impl Pattern for Stripe {
    fn get_transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self.clone()
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        color::Color,
        matrix::Matrix,
        patterns::stripe::Stripe,
        shapes::{sphere::Sphere, Shape},
        tuple::Tuple,
    };

    use super::Pattern;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = Stripe::new(Color::new_white(), Color::new_black());

        assert_eq!(pattern.a, Color::new_white());
        assert_eq!(pattern.b, Color::new_black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Stripe::new(Color::new_white(), Color::new_black());

        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 1., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 2., 0.)),
            Color::new_white()
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Stripe::new(Color::new_white(), Color::new_black());

        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 1.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 2.)),
            Color::new_white()
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Stripe::new(Color::new_white(), Color::new_black());

        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.9, 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(1., 0., 0.)),
            Color::new_black()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(-0.1, 0., 0.)),
            Color::new_black()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(-1., 0., 0.)),
            Color::new_black()
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(-1.1, 0., 0.)),
            Color::new_white()
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Sphere::default().set_transform(Matrix::identity().scaling(2., 2., 2.));
        let pattern = Stripe::new(Color::new_white(), Color::new_black());
        let c = pattern.pattern_at_shape(object.into(), Tuple::point(1.5, 0., 0.));

        assert_eq!(c, Color::new_white());
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::default();
        let pattern = Stripe::new(Color::new_white(), Color::new_black())
            .set_transform(Matrix::identity().scaling(2., 2., 2.));
        let c = pattern.pattern_at_shape(object.into(), Tuple::point(1.5, 0., 0.));

        assert_eq!(c, Color::new_white());
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Sphere::default().set_transform(Matrix::identity().scaling(2., 2., 2.));
        let pattern = Stripe::new(Color::new_white(), Color::new_black())
            .set_transform(Matrix::identity().translation(0.5, 0., 0.));
        let c = pattern.pattern_at_shape(object.into(), Tuple::point(2.5, 0., 0.));

        assert_eq!(c, Color::new_white());
    }
}
