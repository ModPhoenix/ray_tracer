use crate::utils::equal::equal;

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::point::Point;

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
}
