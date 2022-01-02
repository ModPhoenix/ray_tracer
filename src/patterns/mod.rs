use std::rc::Rc;

use crate::{color::Color, matrix::Matrix, shapes::Shape, tuple::Tuple};

pub mod checkers;
pub mod gradient;
pub mod ring;
pub mod stripe;
pub mod test_pattern;

use self::{
    checkers::Checkers, gradient::Gradient, ring::Ring, stripe::Stripe, test_pattern::TestPattern,
};

pub trait Pattern {
    fn get_transform(&self) -> Matrix<4>;
    fn set_transform(&mut self, transform: Matrix<4>) -> Self;

    fn pattern_at(&self, point: Tuple) -> Color;
    fn pattern_at_shape(&self, object: Rc<dyn Shape>, world_point: Tuple) -> Color {
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
    Checkers(Checkers),
    TestPattern(TestPattern),
}

impl Pattern for Patterns {
    fn get_transform(&self) -> Matrix<4> {
        match self {
            Patterns::Stripe(sphere) => sphere.get_transform(),
            Patterns::Gradient(gradient) => gradient.get_transform(),
            Patterns::Ring(ring) => ring.get_transform(),
            Patterns::Checkers(checkers) => checkers.get_transform(),
            Patterns::TestPattern(test_pattern) => test_pattern.get_transform(),
        }
    }

    fn set_transform(&mut self, transform: Matrix<4>) -> Self {
        match self {
            Patterns::Stripe(sphere) => sphere.set_transform(transform).into(),
            Patterns::Gradient(gradient) => gradient.set_transform(transform).into(),
            Patterns::Ring(ring) => ring.set_transform(transform).into(),
            Patterns::Checkers(checkers) => checkers.set_transform(transform).into(),
            Patterns::TestPattern(test_pattern) => test_pattern.set_transform(transform).into(),
        }
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        match self {
            Patterns::Stripe(sphere) => sphere.pattern_at(point),
            Patterns::Gradient(gradient) => gradient.pattern_at(point),
            Patterns::Ring(ring) => ring.pattern_at(point),
            Patterns::Checkers(checkers) => checkers.pattern_at(point),
            Patterns::TestPattern(test_pattern) => test_pattern.pattern_at(point),
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

impl From<Checkers> for Patterns {
    fn from(checkers: Checkers) -> Self {
        Patterns::Checkers(checkers)
    }
}

impl From<TestPattern> for Patterns {
    fn from(test_pattern: TestPattern) -> Self {
        Patterns::TestPattern(test_pattern)
    }
}
