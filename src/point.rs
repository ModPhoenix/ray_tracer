use std::ops::Add;

use crate::{utils::equal::equal, vector::Vector};

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x)
            && equal(self.y, other.y)
            && equal(self.z, other.z)
            && equal(self.w, other.w)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
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
    fn correct_create_new_point() {
        let point = Point::new(4.3, -4.2, 3.1);
        assert_eq!(
            point,
            Point {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 1.0
            }
        );
    }

    #[test]
    fn point_should_always_have_w_equal_1_0() {
        let point = Point::new(4.3, -4.2, 3.1);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn adding_point_and_vector() {
        let point = Point::new(3.0, -2.0, 5.0);
        let vector = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(
            point + vector,
            Point {
                x: 1.0,
                y: 1.0,
                z: 6.0,
                w: 1.0
            }
        );
    }
}
