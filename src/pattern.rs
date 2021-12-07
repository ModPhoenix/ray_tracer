use crate::{
    color::Color,
    matrix::Matrix,
    shape::{Shape, Shapes},
    tuple::Tuple,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    a: Color,
    b: Color,
    transform: Matrix<4>,
}

impl Pattern {
    pub fn stripe_pattern(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Matrix::identity(),
        }
    }

    pub fn set_transform(mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self
    }

    pub fn stripe_at(&self, point: Tuple) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }

    pub fn stripe_at_object(&self, object: Shapes, world_point: Tuple) -> Color {
        let object_point = object.get_transform().inverse() * world_point;
        let pattern_point = self.transform.inverse() * object_point;

        self.stripe_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        color::Color, matrix::Matrix, pattern::Pattern, shape::Shape, sphere::Sphere, tuple::Tuple,
    };

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = Pattern::stripe_pattern(Color::new_white(), Color::new_black());

        assert_eq!(pattern.a, Color::new_white());
        assert_eq!(pattern.b, Color::new_black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripe_pattern(Color::new_white(), Color::new_black());

        assert_eq!(
            pattern.stripe_at(Tuple::point(0., 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.stripe_at(Tuple::point(0., 1., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.stripe_at(Tuple::point(0., 2., 0.)),
            Color::new_white()
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripe_pattern(Color::new_white(), Color::new_black());

        assert_eq!(
            pattern.stripe_at(Tuple::point(0., 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.stripe_at(Tuple::point(0., 0., 1.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.stripe_at(Tuple::point(0., 0., 2.)),
            Color::new_white()
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe_pattern(Color::new_white(), Color::new_black());

        assert_eq!(
            pattern.stripe_at(Tuple::point(0., 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.stripe_at(Tuple::point(0.9, 0., 0.)),
            Color::new_white()
        );
        assert_eq!(
            pattern.stripe_at(Tuple::point(1., 0., 0.)),
            Color::new_black()
        );
        assert_eq!(
            pattern.stripe_at(Tuple::point(-0.1, 0., 0.)),
            Color::new_black()
        );
        assert_eq!(
            pattern.stripe_at(Tuple::point(-1., 0., 0.)),
            Color::new_black()
        );
        assert_eq!(
            pattern.stripe_at(Tuple::point(-1.1, 0., 0.)),
            Color::new_white()
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Sphere::default().set_transform(Matrix::identity().scaling(2., 2., 2.));
        let pattern = Pattern::stripe_pattern(Color::new_white(), Color::new_black());
        let c = pattern.stripe_at_object(object.into(), Tuple::point(1.5, 0., 0.));

        assert_eq!(c, Color::new_white());
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::default();
        let pattern = Pattern::stripe_pattern(Color::new_white(), Color::new_black())
            .set_transform(Matrix::identity().scaling(2., 2., 2.));
        let c = pattern.stripe_at_object(object.into(), Tuple::point(1.5, 0., 0.));

        assert_eq!(c, Color::new_white());
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Sphere::default().set_transform(Matrix::identity().scaling(2., 2., 2.));
        let pattern = Pattern::stripe_pattern(Color::new_white(), Color::new_black())
            .set_transform(Matrix::identity().translation(0.5, 0., 0.));
        let c = pattern.stripe_at_object(object.into(), Tuple::point(2.5, 0., 0.));

        assert_eq!(c, Color::new_white());
    }
}
