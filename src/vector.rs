use crate::utils::equal::equal;

#[derive(Debug)]

pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::vector::Vector;

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
}
