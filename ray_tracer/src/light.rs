use crate::{color::Color, tuple::Tuple};

#[derive(Debug, PartialEq)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, light::Light, tuple::Tuple};

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1., 1., 1.);
        let position = Tuple::point(0., 0., 0.);
        let light = Light::new(position, intensity.clone());

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
