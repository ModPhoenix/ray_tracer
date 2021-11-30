use crate::{
    intersections::{Intersectable, Intersection},
    material::Material,
    ray::Ray,
    sphere::Sphere,
    tuple::Tuple,
    world::Normal,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Sphere(Sphere),
}

impl Object {
    pub fn material(&self) -> &Material {
        match self {
            Object::Sphere(sphere) => &sphere.material,
        }
    }
}

impl Normal for Object {
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        match self {
            Object::Sphere(sphere) => sphere.normal_at(world_point),
        }
    }
}

impl Intersectable for Object {
    fn intersection(&self, t: f64) -> Intersection {
        match self {
            Object::Sphere(sphere) => sphere.intersection(t),
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        match self {
            Object::Sphere(sphere) => sphere.intersect(ray),
        }
    }
}

impl From<Sphere> for Object {
    fn from(sphere: Sphere) -> Self {
        Object::Sphere(sphere)
    }
}
