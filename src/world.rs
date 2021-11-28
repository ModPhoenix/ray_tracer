use crate::tuple::Tuple;

use crate::light::Light;

pub trait Object {
    fn normal_at(&self, world_point: Tuple) -> Tuple;
}

pub struct World<'a> {
    light: Option<Light>,
    objects: Vec<&'a dyn Object>,
}

impl<'a> World<'a> {
    pub fn new(light: Option<Light>, objects: Vec<&'a dyn Object>) -> Self {
        Self { light, objects }
    }
}

impl Default for World<'_> {
    fn default() -> Self {
        Self {
            light: None,
            objects: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, light::Light, material::Material, sphere::Sphere, tuple::Tuple};

    use super::World;

    #[test]
    fn creating_a_world() {
        let w = World::default();

        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }

    // Scenario: The default world
    // Given light ← point_light(point(-10, 10, -10), color(1, 1, 1))
    //     And s1 ← sphere() with:
    //     | material.color     | (0.8, 1.0, 0.6)        |
    //     | material.diffuse   | 0.7                    |
    //     | material.specular  | 0.2                    |
    //     And s2 ← sphere() with:
    //     | transform | scaling(0.5, 0.5, 0.5) |
    // When w ← default_world()
    // Then w.light = light
    //     And w contains s1
    //     And w contains s2

    #[test]
    fn the_default_world() {
        let light = Light::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
        let s1 = Sphere::default().set_material(Material::default());

        let w = World::default();

        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }
}
