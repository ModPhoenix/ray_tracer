pub mod plane;
pub mod sphere;

use crate::{
    intersections::Intersection, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple,
};
use uuid::Uuid;

use self::{plane::Plane, sphere::Sphere};

pub trait Shape {
    fn id(&self) -> Uuid;
    // materials
    fn get_material(&self) -> Material;
    fn set_material(&mut self, material: Material) -> Self;

    // materials
    fn get_transform(&self) -> Matrix<4>;
    fn set_transform(&mut self, transform: Matrix<4>) -> Self;

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

#[derive(Debug, Clone, PartialEq)]
pub enum Shapes {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shapes {
    pub fn get_material(self) -> Material {
        match self {
            Shapes::Sphere(sphere) => sphere.material,
            Shapes::Plane(plane) => plane.material,
        }
    }
}

impl Shape for Shapes {
    fn id(&self) -> Uuid {
        match self {
            Shapes::Sphere(sphere) => sphere.id(),
            Shapes::Plane(plane) => plane.id(),
        }
    }

    fn get_material(&self) -> Material {
        match self {
            Shapes::Sphere(sphere) => sphere.get_material(),
            Shapes::Plane(plane) => plane.get_material(),
        }
    }

    fn set_material(&mut self, material: Material) -> Self {
        match self {
            Shapes::Sphere(sphere) => sphere.set_material(material).into(),
            Shapes::Plane(plane) => plane.set_material(material).into(),
        }
    }

    fn get_transform(&self) -> crate::matrix::Matrix<4> {
        match self {
            Shapes::Sphere(sphere) => sphere.get_transform(),
            Shapes::Plane(plane) => plane.get_transform(),
        }
    }

    fn set_transform(&mut self, transform: crate::matrix::Matrix<4>) -> Self {
        match self {
            Shapes::Sphere(sphere) => sphere.set_transform(transform).into(),
            Shapes::Plane(plane) => plane.set_transform(transform).into(),
        }
    }

    fn intersection(&self, t: f64) -> Intersection {
        match self {
            Shapes::Sphere(sphere) => sphere.intersection(t),
            Shapes::Plane(plane) => plane.intersection(t),
        }
    }

    fn local_intersect(&self, local_ray: &Ray) -> Option<Vec<Intersection>> {
        match self {
            Shapes::Sphere(sphere) => sphere.local_intersect(local_ray),
            Shapes::Plane(plane) => plane.local_intersect(local_ray),
        }
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        match self {
            Shapes::Sphere(sphere) => sphere.local_normal_at(local_point),
            Shapes::Plane(plane) => plane.local_normal_at(local_point),
        }
    }
}

impl From<Sphere> for Shapes {
    fn from(sphere: Sphere) -> Self {
        Shapes::Sphere(sphere)
    }
}

impl From<Plane> for Shapes {
    fn from(plane: Plane) -> Self {
        Shapes::Plane(plane)
    }
}
