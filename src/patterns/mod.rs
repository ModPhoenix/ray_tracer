pub mod gradient;
pub mod ring;
pub mod stripe;

use crate::{
    color::Color,
    matrix::Matrix,
    shapes::{Shape, Shapes},
    tuple::Tuple,
};

use self::{gradient::Gradient, ring::Ring, stripe::Stripe};

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
    Gradient(Gradient),
    Ring(Ring),
}

impl Pattern for Patterns {
    fn get_transform(&self) -> Matrix<4> {
        match self {
            Patterns::Stripe(sphere) => sphere.get_transform(),
            Patterns::Gradient(gradient) => gradient.get_transform(),
            Patterns::Ring(ring) => ring.get_transform(),
        }
    }

    fn set_transform(&mut self, transform: Matrix<4>) -> Self {
        match self {
            Patterns::Stripe(sphere) => sphere.set_transform(transform).into(),
            Patterns::Gradient(gradient) => gradient.set_transform(transform).into(),
            Patterns::Ring(ring) => ring.set_transform(transform).into(),
        }
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        match self {
            Patterns::Stripe(sphere) => sphere.pattern_at(point),
            Patterns::Gradient(gradient) => gradient.pattern_at(point),
            Patterns::Ring(ring) => ring.pattern_at(point),
        }
    }
}

impl From<Stripe> for Patterns {
    fn from(sphere: Stripe) -> Self {
        Patterns::Stripe(sphere)
    }
}

impl From<Gradient> for Patterns {
    fn from(gradient: Gradient) -> Self {
        Patterns::Gradient(gradient)
    }
}

impl From<Ring> for Patterns {
    fn from(ring: Ring) -> Self {
        Patterns::Ring(ring)
    }
}
