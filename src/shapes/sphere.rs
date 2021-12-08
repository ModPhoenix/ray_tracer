use crate::{intersections::Intersection, material::Material, matrix::Matrix, tuple::Tuple};

use super::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Sphere {
    pub fn new(transform: Matrix<4>, material: Material) -> Self {
        Self {
            transform,
            material,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(Matrix::identity(), Material::default())
    }
}

impl Shape for Sphere {
    fn get_material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, material: Material) -> Self {
        self.material = material;
        self.clone()
    }

    fn get_transform(&self) -> Matrix<4> {
        self.transform.clone()
    }

    fn set_transform(&mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self.clone()
    }

    fn intersection(&self, t: f64) -> Intersection {
        Intersection::new(t, self.clone().into())
    }

    fn local_intersect(&self, local_ray: &crate::ray::Ray) -> Option<Vec<Intersection>> {
        let sphere_to_ray = local_ray.origin - Tuple::point(0., 0., 0.);
        let a = Tuple::dot(&local_ray.direction, &local_ray.direction);
        let b = 2.0 * Tuple::dot(&local_ray.direction, &sphere_to_ray);
        let c = Tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            return Some(vec![self.intersection(t1), self.intersection(t2)]);
        }
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        local_point - Tuple::point(0., 0., 0.)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        material::Material,
        matrix::Matrix,
        ray::Ray,
        shapes::{sphere::Sphere, Shape},
        tuple::Tuple,
    };

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let xs = Sphere::default().intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].t, 4.0);
        assert_eq!(xs.unwrap()[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let xs = Sphere::default().intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].t, 5.0);
        assert_eq!(xs.unwrap()[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let xs = Sphere::default().intersect(&r);

        assert_eq!(xs, None);
    }

    #[test]
    fn a_ra_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let xs = Sphere::default().intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].t, -1.0);
        assert_eq!(xs.unwrap()[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let xs = Sphere::default().intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].t, -6.0);
        assert_eq!(xs.unwrap()[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = Sphere::default().intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].object, s.clone().into());
        assert_eq!(xs.unwrap()[1].object, s.into());
    }

    #[test]
    fn a_sphere_default_transformation() {
        let s = Sphere::default();

        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let t = Matrix::identity().translation(2., 3., 4.);
        let s = Sphere::default().set_transform(t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default().set_transform(Matrix::identity().scaling(2., 2., 2.));

        let xs = s.intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 2);
        assert_eq!(xs.clone().unwrap()[0].t, 3.);
        assert_eq!(xs.unwrap()[1].t, 7.);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default().set_transform(Matrix::identity().translation(5., 0., 0.));

        let xs = s.intersect(&r);

        assert_eq!(xs, None);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(1., 0., 0.));

        assert_eq!(n, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(0., 1., 0.));

        assert_eq!(n, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(0., 0., 1.));

        assert_eq!(n, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::default();

        let value = 3.0_f64.sqrt() / 3.;

        let n = s.normal_at(Tuple::point(value, value, value));

        assert_eq!(n, Tuple::vector(value, value, value));
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::default();

        let value = 3.0_f64.sqrt() / 3.;

        let n = s.normal_at(Tuple::point(value, value, value));

        assert_eq!(n, Tuple::vector(value, value, value).normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::default().set_transform(Matrix::identity().translation(0., 1., 0.));

        let n = s.normal_at(Tuple::point(0., 1.70711, -0.70711));

        assert_eq!(n, Tuple::vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Sphere::default()
            .set_transform(Matrix::identity().rotation_z(PI / 5.).scaling(1., 0.5, 1.));
        let n = s.normal_at(Tuple::point(0., 2.0_f64.sqrt() / 2., -2.0_f64.sqrt() / 2.));

        assert_eq!(n, Tuple::vector(0., 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::default();

        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::default();
        let m = Material::default().set_ambient(1.).clone();

        s.material = m.clone();

        assert_eq!(s.material, m);
    }
}
