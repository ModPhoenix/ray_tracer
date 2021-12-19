use std::ops::{Add, Mul, Sub};

use crate::utils::fuzzy_equal::fuzzy_equal;

use super::RGB;

#[derive(Debug, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn new_black() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn new_white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    /// Get a reference to the color's red.
    pub fn red(&self) -> f64 {
        self.red
    }

    /// Get a reference to the color's green.
    pub fn green(&self) -> f64 {
        self.green
    }

    /// Get a reference to the color's blue.
    pub fn blue(&self) -> f64 {
        self.blue
    }

    pub fn clamp(x: f64) -> f64 {
        match x {
            x if x > 1.0 => 1.0,
            x if x < 0.0 => 0.0,
            _ => x,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        fuzzy_equal(self.red, other.red)
            && fuzzy_equal(self.green, other.green)
            && fuzzy_equal(self.blue, other.blue)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

fn rgb_value_to_color(value: u8) -> f64 {
    (value as f64 * 100.) / (255. * 100.)
}

impl From<RGB> for Color {
    fn from(rgb: RGB) -> Self {
        Color::new(
            rgb_value_to_color(rgb.red()),
            rgb_value_to_color(rgb.green()),
            rgb_value_to_color(rgb.blue()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn create_new_black_color() {
        let c = Color::new_black();

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }
}
