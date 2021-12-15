use crate::color::Color;
use crate::intersections::{ComputedIntersection, Intersections};
use crate::ray::Ray;

use crate::shapes::{Shape, Shapes};
use crate::tuple::Tuple;

use crate::light::Light;

#[derive(Debug, Clone)]
pub struct World {
    light: Option<Light>,
    objects: Vec<Shapes>,
}

impl World {
    pub fn new(light: Option<Light>, objects: Vec<Shapes>) -> Self {
        Self { light, objects }
    }

    pub fn set_light(mut self, light: Light) -> Self {
        self.light = Some(light);

        self
    }

    pub fn intersect_world(&self, ray: &Ray) -> Intersections {
        let xs = self.objects.iter().fold(vec![], |mut acc, object| {
            if let Some(intersection) = object.intersect(ray) {
                acc.extend(intersection);
            }
            acc
        });

        Intersections::new(xs)
    }

    // TODO: add support multiple light sources
    pub fn shade_hit(&self, comps: ComputedIntersection, remaining: usize) -> Color {
        let is_shadowed = self.is_shadowed(comps.over_point);
        let surface = comps.object.clone().get_material().lighting(
            comps.object.clone(),
            self.light.clone().unwrap(),
            comps.over_point,
            comps.eyev,
            comps.normalv,
            is_shadowed,
        );
        let reflected = self.reflected_color(comps, remaining);

        surface + reflected
    }

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> Color {
        let xs = self.intersect_world(ray);
        match xs.hit() {
            Some(intersection) => {
                let comps = intersection.prepare_computations(ray, Intersections::default());
                self.shade_hit(comps, remaining)
            }
            None => Color::new_black(),
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let v = self.light.as_ref().unwrap().position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let intersections = self.intersect_world(&r);

        let h = intersections.hit();

        if let Some(intersection) = h {
            if intersection.t < distance {
                return true;
            }
        }

        return false;
    }

    pub fn reflected_color(&self, comps: ComputedIntersection, remaining: usize) -> Color {
        if remaining <= 0 || comps.object.clone().get_material().get_reflective() == 0. {
            return Color::new_black();
        }

        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray, remaining - 1);

        return color * comps.object.get_material().get_reflective();
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
        color::Color,
        intersections::{Intersection, Intersections},
        light::Light,
        material::Material,
        matrix::Matrix,
        ray::Ray,
        shapes::{plane::Plane, sphere::Sphere, Shape},
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
        let xs = w.intersect_world(&r);

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
        let comps = i.prepare_computations(&r, Intersections::default());

        let c = w.shade_hit(comps, 5);

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
        let comps = i.prepare_computations(&r, Intersections::default());

        let c = w.shade_hit(comps, 5);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));

        let c = w.color_at(&r, 5);

        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let c = w.color_at(&r, 5);

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

        let c = w.clone().color_at(&r, 5);

        assert_eq!(c, inner.clone().get_material().get_color());
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = Tuple::point(0., 10., 0.);

        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = Tuple::point(10., -10., 10.);

        assert_eq!(w.is_shadowed(p), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = Tuple::point(-20., 20., -20.);

        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        let p = Tuple::point(-2., 2., -2.);

        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let light = Light::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let s1 = Sphere::default();
        let s2 = Sphere::default().set_transform(Matrix::identity().translation(0., 0., 10.));
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let i = Intersection::new(4., s2.clone().into());
        let comps = i.prepare_computations(&r, Intersections::default());

        let w = World::new(Some(light), vec![s1.into(), s2.into()]);
        let c = w.shade_hit(comps, 5);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut w = default_world();
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));

        let second_object = w.objects[1].clone();
        w.objects[1].set_material(second_object.get_material().set_ambient(1.));

        let i = w.objects[1].intersection(1.);
        let comps = i.prepare_computations(&r, Intersections::default());
        let color = w.reflected_color(comps, 5);

        assert_eq!(color, Color::new_black());
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = default_world();
        let shape = Plane::default()
            .set_material(Material::default().set_reflective(0.5))
            .set_transform(Matrix::identity().translation(0., -1., 0.));

        w.objects.push(shape.into());

        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2.0_f64.sqrt() / 2., 2.0_f64.sqrt() / 2.),
        );

        let i = w.objects[2].intersection(2.0_f64.sqrt());
        let comps = i.prepare_computations(&r, Intersections::default());
        let color = w.reflected_color(comps, 5);

        assert_eq!(color, Color::new(0.190332, 0.237915, 0.142749));
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = default_world();
        let shape = Plane::default()
            .set_material(Material::default().set_reflective(0.5))
            .set_transform(Matrix::identity().translation(0., -1., 0.));

        w.objects.push(shape.into());

        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2.0_f64.sqrt() / 2., 2.0_f64.sqrt() / 2.),
        );

        let i = w.objects[2].intersection(2.0_f64.sqrt());
        let comps = i.prepare_computations(&r, Intersections::default());
        let color = w.shade_hit(comps, 5);

        assert_eq!(color, Color::new(0.87675, 0.92434, 0.82917));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w =
            World::default().set_light(Light::new(Tuple::point(0., 0., 0.), Color::new_white()));
        let lower = Plane::default()
            .set_material(Material::default().set_reflective(1.))
            .set_transform(Matrix::identity().translation(0., -1., 0.));
        let upper = Plane::default()
            .set_material(Material::default().set_reflective(1.))
            .set_transform(Matrix::identity().translation(0., 1., 0.));

        w.objects.push(lower.into());
        w.objects.push(upper.into());

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.));

        w.color_at(&r, 5);

        assert!(true);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();
        let shape = Plane::default()
            .set_material(Material::default().set_reflective(0.5))
            .set_transform(Matrix::identity().translation(0., -1., 0.));

        w.objects.push(shape.into());

        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2.0_f64.sqrt() / 2., 2.0_f64.sqrt() / 2.),
        );

        let i = w.objects[2].intersection(2.0_f64.sqrt());
        let comps = i.prepare_computations(&r, Intersections::default());
        let color = w.reflected_color(comps, 0);

        assert_eq!(color, Color::new_black());
    }
}
