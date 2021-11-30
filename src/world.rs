use crate::intersections::{Intersectable, Intersections};
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;

use crate::light::Light;

pub trait Normal {
    fn normal_at(&self, world_point: Tuple) -> Tuple;
}

pub struct World {
    light: Option<Light>,
    objects: Vec<Object>,
}

impl World {
    pub fn new(light: Option<Light>, objects: Vec<Object>) -> Self {
        Self { light, objects }
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let xs = self.objects.iter().fold(vec![], |mut acc, object| {
            if let Some(intersection) = object.intersect(ray) {
                acc.extend(intersection);
            }
            acc
        });

        Intersections::new(xs)
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            light: None,
            objects: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color, light::Light, material::Material, matrix::Matrix, ray::Ray, sphere::Sphere,
        tuple::Tuple,
    };

    use super::World;

    fn default_world() -> World {
        let light = Light::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
        let s1 = Sphere::default().set_material(
            Material::default()
                .set_color(Color::new(0.8, 1.0, 0.6))
                .set_diffuse(0.7)
                .set_specular(0.2),
        );
        let s2 = Sphere::default().set_transform(Matrix::identity().scaling(0.5, 0.5, 0.5));

        World::new(Some(light), vec![s1.into(), s2.into()])
    }

    #[test]
    fn creating_a_world() {
        let w = World::default();

        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }

    #[test]
    fn the_default_world() {
        let light = Light::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
        let _s1 = Sphere::default().set_material(
            Material::default()
                .set_color(Color::new(0.8, 1.0, 0.6))
                .set_diffuse(0.7)
                .set_specular(0.2),
        );
        let _s2 = Sphere::default().set_transform(Matrix::identity().scaling(0.5, 0.5, 0.5));

        let w = default_world();

        assert_eq!(w.light.unwrap(), light);
        assert_eq!(w.objects.len(), 2);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let xs = w.intersect(&r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.);
    }
}
