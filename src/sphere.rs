use crate::{
    intersections::{Intersectable, Intersection},
    matrix::Matrix,
    ray::Ray,
    tuple::Tuple,
};

#[derive(Debug, Clone, PartialEq)]

pub struct Sphere {
    pub transform: Matrix<4>,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transform: Matrix::identity(),
        }
    }

    pub fn set_transform(&mut self, matrix: Matrix<4>) {
        self.transform = matrix
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - Tuple::point(0., 0., 0.);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;

        world_normal.w = 0.;

        world_normal.normalize()
    }
}

impl Intersectable<Sphere> for Sphere {
    fn intersection(&self, t: f64) -> Intersection<Sphere> {
        Intersection::new(t, self.clone())
    }

    fn intersect(&self, ray: &Ray) -> Option<[Intersection<Self>; 2]> {
        let ray_transformed = ray.transform(self.transform.inverse());

        let sphere_to_ray = ray_transformed.origin - Tuple::point(0., 0., 0.);
        let a = Tuple::dot(&ray_transformed.direction, &ray_transformed.direction);
        let b = 2.0 * Tuple::dot(&ray_transformed.direction, &sphere_to_ray);
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
    use std::f64::consts::PI;

    use crate::{
        intersections::Intersectable, matrix::Matrix, ray::Ray, sphere::Sphere, tuple::Tuple,
    };

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

    #[test]
    fn a_sphere_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let mut s = Sphere::new();
        let t = Matrix::identity().translation(2., 3., 4.);
        s.set_transform(t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = Sphere::new();
        s.set_transform(Matrix::identity().scaling(2., 2., 2.));

        let xs = s.intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].t, 3.);
        assert_eq!(xs.unwrap()[1].t, 7.);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = Sphere::new();
        s.set_transform(Matrix::identity().translation(5., 0., 0.));

        let xs = s.intersect(&r);

        assert_eq!(xs, None);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple::point(1., 0., 0.));

        assert_eq!(n, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple::point(0., 1., 0.));

        assert_eq!(n, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple::point(0., 0., 1.));

        assert_eq!(n, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();

        let value = 3.0_f64.sqrt() / 3.;

        let n = s.normal_at(Tuple::point(value, value, value));

        assert_eq!(n, Tuple::vector(value, value, value));
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();

        let value = 3.0_f64.sqrt() / 3.;

        let n = s.normal_at(Tuple::point(value, value, value));

        assert_eq!(n, Tuple::vector(value, value, value).normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::identity().translation(0., 1., 0.));

        let n = s.normal_at(Tuple::point(0., 1.70711, -0.70711));

        assert_eq!(n, Tuple::vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::identity().rotation_z(PI / 5.).scaling(1., 0.5, 1.));

        let n = s.normal_at(Tuple::point(0., 2.0_f64.sqrt() / 2., -2.0_f64.sqrt() / 2.));

        assert_eq!(n, Tuple::vector(0., 0.97014, -0.24254));
    }
}
