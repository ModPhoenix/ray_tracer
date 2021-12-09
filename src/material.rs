use crate::{
    color::Color,
    light::Light,
    patterns::{Pattern, Patterns},
    shapes::Shapes,
    tuple::Tuple,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
    pattern: Option<Patterns>,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
        transparency: f64,
        refractive_index: f64,
        pattern: Option<Patterns>,
    ) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            reflective,
            transparency,
            refractive_index,
            pattern,
        }
    }

    pub fn get_color(self) -> Color {
        self.color
    }

    pub fn set_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn set_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn set_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn set_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn set_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn get_reflective(&self) -> f64 {
        self.reflective
    }

    pub fn set_reflective(mut self, reflective: f64) -> Self {
        self.reflective = reflective;
        self
    }

    pub fn set_pattern(mut self, pattern: Patterns) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn get_transparency(&self) -> f64 {
        self.transparency
    }

    pub fn set_transparency(mut self, transparency: f64) -> Self {
        self.transparency = transparency;
        self
    }

    pub fn get_refractive_index(&self) -> f64 {
        self.refractive_index
    }

    pub fn set_refractive_index(mut self, refractive_index: f64) -> Self {
        self.refractive_index = refractive_index;
        self
    }

    pub fn lighting(
        &self,
        object: Shapes,
        light: Light,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        let ambient: Color;
        let diffuse: Color;
        let specular: Color;
        let color: Color;

        if let Some(pattern) = self.pattern.clone() {
            color = pattern.pattern_at_shape(object, point);
        } else {
            color = self.color.clone();
        }

        let effective_color = color * light.intensity.clone();
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

        if in_shadow {
            return ambient;
        }

        return ambient + diffuse + specular;
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.,
            transparency: 0.,
            refractive_index: 1.,
            pattern: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color, light::Light, material::Material, patterns::stripe::Stripe,
        shapes::sphere::Sphere, tuple::Tuple,
    };

    #[test]
    fn the_default_material() {
        let m = Material::default();

        assert_eq!(m.color, Color::new(1., 1., 1.));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.);
    }

    #[test]
    fn reflectivity_for_the_default_material() {
        let m = Material::default();

        assert_eq!(m.reflective, 0.);
    }

    #[test]
    fn transparency_and_refractive_index_for_the_default_material() {
        let m = Material::default();

        assert_eq!(m.transparency, 0.);
        assert_eq!(m.refractive_index, 1.);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));

        let result = m.lighting(
            Sphere::default().into(),
            light,
            position,
            eyev,
            normalv,
            false,
        );

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 2.0_f64.sqrt() / 2., -2.0_f64.sqrt() / 2.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));

        let result = m.lighting(
            Sphere::default().into(),
            light,
            position,
            eyev,
            normalv,
            false,
        );

        assert_eq!(result, Color::new(1., 1., 1.));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));

        let result = m.lighting(
            Sphere::default().into(),
            light,
            position,
            eyev,
            normalv,
            false,
        );

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., -2.0_f64.sqrt() / 2., -2.0_f64.sqrt() / 2.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));

        let result = m.lighting(
            Sphere::default().into(),
            light,
            position,
            eyev,
            normalv,
            false,
        );

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 0., 10.), Color::new(1., 1., 1.));

        let result = m.lighting(
            Sphere::default().into(),
            light,
            position,
            eyev,
            normalv,
            false,
        );

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let in_shadow = true;

        let result = m.lighting(
            Sphere::default().into(),
            light,
            position,
            eyev,
            normalv,
            in_shadow,
        );

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let m = Material::default()
            .set_pattern(Stripe::new(Color::new_white(), Color::new_black()).into())
            .set_ambient(1.)
            .set_diffuse(0.)
            .set_specular(0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Light::new(Tuple::point(0., 0., -10.), Color::new_white());

        let c1 = m.lighting(
            Sphere::default().into(),
            light.clone(),
            Tuple::point(0.9, 0., 0.),
            eyev,
            normalv,
            false,
        );
        let c2 = m.lighting(
            Sphere::default().into(),
            light,
            Tuple::point(1.1, 0., 0.),
            eyev,
            normalv,
            false,
        );

        assert_eq!(c1, Color::new_white());
        assert_eq!(c2, Color::new_black());
    }
}
