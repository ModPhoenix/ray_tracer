use crate::{color::Color, tuple::Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    a: Color,
    b: Color,
}

impl Pattern {
    pub fn stripe_pattern(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn stripe_at(&self, point: Tuple) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{color::Color, pattern::Pattern, tuple::Tuple};

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
}
