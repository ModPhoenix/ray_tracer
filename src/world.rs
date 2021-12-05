use crate::color::Color;
use crate::intersections::{ComputedIntersection, Intersectable, Intersections};
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;

use crate::light::Light;

pub trait Normal {
    fn normal_at(&self, world_point: Tuple) -> Tuple;
}

#[derive(Debug, Clone)]
pub struct World {
    light: Option<Light>,
    objects: Vec<Object>,
}

impl World {
    pub fn new(light: Option<Light>, objects: Vec<Object>) -> Self {
        Self { light, objects }
    }

    pub fn set_light(mut self, light: Light) -> Self {
        self.light = Some(light);

        self
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

    // TODO: add support multiple light sources
    pub fn shade_hit(&self, comps: ComputedIntersection) -> Color {
        comps.object.get_material().lighting(
            self.light.clone().unwrap(),
            comps.point,
            comps.eyev,
            comps.normalv,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect(ray);
        match xs.hit() {
            Some(intersection) => {
                let comps = intersection.prepare_computations(ray);
                self.shade_hit(comps)
            }
            None => Color::new_black(),
        }
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
        color::Color, intersections::Intersectable, light::Light, material::Material,
        matrix::Matrix, ray::Ray, sphere::Sphere, tuple::Tuple,
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

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let shape = &w.objects[0];
        let i = shape.intersection(4.);
        let comps = i.prepare_computations(&r);

        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = default_world().set_light(Light::new(
            Tuple::point(0., 0.25, 0.),
            Color::new(1., 1., 1.),
        ));

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));

        let shape = &w.objects[1];
        let i = shape.intersection(0.5);
        let comps = i.prepare_computations(&r);

        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));

        let c = w.color_at(&r);

        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let c = w.color_at(&r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let light = Light::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
        let s1 = Sphere::default().set_material(
            Material::default()
                .set_color(Color::new(0.8, 1.0, 0.6))
                .set_diffuse(0.7)
                .set_specular(0.2)
                .set_ambient(1.),
        );
        let s2 = Sphere::default()
            .set_transform(Matrix::identity().scaling(0.5, 0.5, 0.5))
            .set_material(Material::default().set_ambient(1.));

        let w = World::new(Some(light), vec![s1.into(), s2.into()]);

        let inner = &w.objects[1];

        let r = Ray::new(Tuple::point(0., 0., 0.75), Tuple::vector(0., 0., -1.));

        let c = w.clone().color_at(&r);

        assert_eq!(c, inner.clone().get_material().clone().get_color());
    }
}