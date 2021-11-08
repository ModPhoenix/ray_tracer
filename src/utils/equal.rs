use crate::constants::EPSILON;

pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use crate::utils::equal::equal;

    #[test]
    fn should_be_equal() {
        assert!(equal(1.0, 1.0));
        assert!(equal(0.1, 0.1));
        assert!(equal(0.00005, 0.00006));
        assert!(equal(0.0000009, -0.0000001));
        assert!(equal(0.0000009234234, -0.0000001234234));
    }

    #[test]
    fn should_not_be_equal() {
        assert!(!equal(1.0, -1.0));
        assert!(!equal(-0.1, 0.1));
        assert!(!equal(0.0005, 0.0006));
    }
}
