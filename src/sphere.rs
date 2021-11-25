use crate::{
    intersections::{Intersectable, Intersection},
    ray::Ray,
    tuple::Tuple,
};

#[derive(Debug, Clone, PartialEq)]

pub struct Sphere;

impl Sphere {
    pub fn new() -> Self {
        Self
    }
}

impl Intersectable<Sphere> for Sphere {
    fn intersection(&self, t: f64) -> Intersection<Sphere> {
        Intersection::new(t, self.clone())
    }

    fn intersect(&self, ray: &Ray) -> Option<[Intersection<Self>; 2]> {
        let sphere_to_ray = ray.origin - Tuple::point(0., 0., 0.);
        let a = Tuple::dot(&ray.direction, &ray.direction);
        let b = 2.0 * Tuple::dot(&ray.direction, &sphere_to_ray);
        let c = Tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            return Some([self.intersection(t1), self.intersection(t2)]);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{intersections::Intersectable, ray::Ray, sphere::Sphere, tuple::Tuple};

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let xs = Sphere::new().intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].t, 4.0);
        assert_eq!(xs.unwrap()[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let xs = Sphere::new().intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].t, 5.0);
        assert_eq!(xs.unwrap()[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let xs = Sphere::new().intersect(&r);

        assert_eq!(xs, None);
    }

    #[test]
    fn a_ra_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let xs = Sphere::new().intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].t, -1.0);
        assert_eq!(xs.unwrap()[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let xs = Sphere::new().intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].t, -6.0);
        assert_eq!(xs.unwrap()[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = Sphere::new().intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].object, s);
        assert_eq!(xs.unwrap()[1].object, s);
    }
}
