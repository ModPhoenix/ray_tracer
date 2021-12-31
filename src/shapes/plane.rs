use uuid::Uuid;

use crate::{
    constants::EPSILON, intersections::Intersection, material::Material, matrix::Matrix,
    tuple::Tuple,
};

use super::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    id: Uuid,
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Plane {
    pub fn new(transform: Matrix<4>, material: Material) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform,
            material,
        }
    }

    pub fn set_material(&mut self, material: Material) -> Self {
        self.material = material;
        self.clone()
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self.clone()
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane::new(Matrix::identity(), Material::default())
    }
}

impl Shape for Plane {
    fn id(&self) -> Uuid {
        self.id
    }

    fn get_material(&self) -> Material {
        self.material.clone()
    }

    fn get_transform(&self) -> Matrix<4> {
        self.transform.clone()
    }

    fn intersection(&self, t: f64) -> Intersection {
        Intersection::new(t, self.clone().into())
    }

    fn local_intersect(&self, ray: &crate::ray::Ray) -> Option<Vec<Intersection>> {
        if ray.direction.y.abs() < EPSILON {
            return None;
        }

        let t = -ray.origin.y / ray.direction.y;

        Some(vec![self.intersection(t)])
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        Tuple::vector(0., 1., 0.)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        ray::Ray,
        shapes::{plane::Plane, Shape},
        tuple::Tuple,
    };

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::default();

        let n1 = p.local_normal_at(Tuple::point(0., 0., 0.));
        let n2 = p.local_normal_at(Tuple::point(10., 0., -10.));
        let n3 = p.local_normal_at(Tuple::point(-5., 0., 150.));

        assert_eq!(n1, Tuple::vector(0., 1., 0.));
        assert_eq!(n2, Tuple::vector(0., 1., 0.));
        assert_eq!(n3, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::default();

        let r = Ray::new(Tuple::point(0., 10., 0.), Tuple::vector(0., 0., 1.));
        let xs = p.local_intersect(&r);

        assert_eq!(xs, None);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::default();

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let xs = p.local_intersect(&r);

        assert_eq!(xs, None);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::default();

        let r = Ray::new(Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.));
        let xs = p.local_intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 1);
        assert_eq!(xs.clone().unwrap()[0].t, 1.);
        assert_eq!(xs.unwrap()[0].object, p.into());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::default();

        let r = Ray::new(Tuple::point(0., -1., 0.), Tuple::vector(0., 1., 0.));
        let xs = p.local_intersect(&r);

        assert_eq!(xs.clone().unwrap().len(), 1);
        assert_eq!(xs.clone().unwrap()[0].t, 1.);
        assert_eq!(xs.unwrap()[0].object, p.into());
    }
}
