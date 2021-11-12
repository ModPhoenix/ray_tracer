use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utils::equal::equal;

#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x)
            && equal(self.y, other.y)
            && equal(self.z, other.z)
            && equal(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tuple;

    #[test]
    fn tuple_with_w_1_0_is_point() {
        let point = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert!(point.is_point());
        assert!(!point.is_vector());
    }

    #[test]
    fn tuple_with_w_0_0_is_vector() {
        let vector = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert!(!vector.is_point());
        assert!(vector.is_vector());
    }

    #[test]
    fn point_creates_tuples_with_w_1_0() {
        let point = Tuple::point(4.3, -4.2, 3.1);

        assert_eq!(point, Tuple::new(4.3, -4.2, 3.1, 1.0));
    }

    #[test]
    fn vector_creates_tuples_with_w_0_0() {
        let vector = Tuple::vector(4.3, -4.2, 3.1);

        assert_eq!(vector, Tuple::new(4.3, -4.2, 3.1, 0.0));
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }
}