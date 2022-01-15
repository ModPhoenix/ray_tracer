use std::fmt::Debug;
use uuid::Uuid;

use crate::{
    intersections::Intersection, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple,
};

pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod sphere;

pub trait Shape: Sync + Debug {
    fn id(&self) -> Uuid;
    // materials
    fn get_material(&self) -> Material;
    /// Set the Shape's material.
    fn set_material(&mut self, material: Material);

    // transform
    fn get_transform(&self) -> Matrix<4>;
    /// Set the Shape's transform.
    fn set_transform(&mut self, transform: Matrix<4>);

    // intersection
    fn intersection(&self, t: f64) -> Intersection;
    fn local_intersect(&self, local_ray: &Ray) -> Option<Vec<Intersection>>;
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let local_ray = ray.transform(self.get_transform().inverse());
        self.local_intersect(&local_ray)
    }

    // normal
    fn local_normal_at(&self, local_point: Tuple) -> Tuple;
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let local_point = self.get_transform().inverse() * world_point;
        let local_normal = self.local_normal_at(local_point);
        let mut world_normal = self.get_transform().inverse().transpose() * local_normal;

        world_normal.w = 0.;

        world_normal.normalize()
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
