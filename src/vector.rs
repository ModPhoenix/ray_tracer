use std::ops::Add;

use crate::{point::Point, utils::equal::equal};

#[derive(Debug)]

pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x)
            && equal(self.y, other.y)
            && equal(self.z, other.z)
            && equal(self.w, other.w)
    }
}

impl Add for Vector {
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

impl Add<Point> for Vector {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{point::Point, vector::Vector};

    #[test]
    fn correct_create_new_vector() {
        let vector = Vector::new(4.3, -4.2, 3.1);

        assert_eq!(
            vector,
            Vector {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 0.0
            }
        );
    }

    #[test]
    fn vector_should_always_have_w_equal_0_0() {
        let vector = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(vector.w, 0.0);
    }

    #[test]
    fn adding_two_vectors() {
        let vector1 = Vector::new(3.0, -2.0, 5.0);
        let vector2 = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(
            vector1 + vector2,
            Vector {
                x: 1.0,
                y: 1.0,
                z: 6.0,
                w: 0.0
            }
        );
    }

    #[test]
    fn adding_vector_and_point() {
        let vector = Vector::new(3.0, -2.0, 5.0);
        let point = Point::new(-2.0, 3.0, 1.0);
        assert_eq!(
            vector + point,
            Vector {
                x: 1.0,
                y: 1.0,
                z: 6.0,
                w: 1.0
            }
        );
    }
}
