pub mod stripe;

use crate::{
    color::Color,
    matrix::Matrix,
    shapes::{Shape, Shapes},
    tuple::Tuple,
};

use self::stripe::Stripe;

pub trait Pattern {
    fn get_transform(&self) -> Matrix<4>;
    fn set_transform(&mut self, transform: Matrix<4>) -> Self;

    fn pattern_at(&self, point: Tuple) -> Color;
    fn pattern_at_shape(&self, object: Shapes, world_point: Tuple) -> Color {
        let object_point = object.get_transform().inverse() * world_point;
        let pattern_point = self.get_transform().inverse() * object_point;

        self.pattern_at(pattern_point)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Patterns {
    Stripe(Stripe),
}

impl Pattern for Patterns {
    fn get_transform(&self) -> Matrix<4> {
        match self {
            Patterns::Stripe(sphere) => sphere.get_transform(),
        }
    }

    fn set_transform(&mut self, transform: Matrix<4>) -> Self {
        match self {
            Patterns::Stripe(sphere) => sphere.set_transform(transform).into(),
        }
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        match self {
            Patterns::Stripe(sphere) => sphere.pattern_at(point),
        }
    }
}

impl From<Stripe> for Patterns {
    fn from(sphere: Stripe) -> Self {
        Patterns::Stripe(sphere)
    }
}
