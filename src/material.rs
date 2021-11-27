use crate::{color::Color, light::Light, tuple::Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::new(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(self, light: Light, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        let ambient: Color;
        let diffuse: Color;
        let specular: Color;

        let effective_color = self.color * light.intensity.clone();
        let lightv = (light.position - point).normalize();

        ambient = effective_color.clone() * self.ambient;

        let light_dot_normal = Tuple::dot(&lightv, &normalv);

        if light_dot_normal < 0. {
            diffuse = Color::new_black();
            specular = Color::new_black();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = Tuple::dot(&reflectv, &eyev);

            if reflect_dot_eye <= 0. {
                specular = Color::new_black();
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);

                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, light::Light, material::Material, tuple::Tuple};

    #[test]
    fn the_default_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::new(1., 1., 1.));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 2.0_f64.sqrt() / 2., -2.0_f64.sqrt() / 2.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Color::new(1., 1., 1.));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., -2.0_f64.sqrt() / 2., -2.0_f64.sqrt() / 2.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 0., 10.), Color::new(1., 1., 1.));

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
