use uuid::Uuid;

use crate::{
    intersections::Intersection, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple,
};

use self::{plane::Plane, sphere::Sphere};

// pub mod cone;
// pub mod cube;
// pub mod cylinder;
pub mod plane;
pub mod sphere;

pub trait Shape {
    fn id(&self) -> Uuid;
    // materials
    fn get_material(&self) -> Material;
    // fn set_material(&mut self, material: Material) -> Self;

    // transform
    fn get_transform(&self) -> Matrix<4>;
    // fn set_transform(&mut self, transform: Matrix<4>) -> Self;

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
    // Cube(Cube),
    // Cylinder(Cylinder),
    // Cone(Cone),
}

impl Shapes {
    pub fn get_material(self) -> Material {
        match self {
            Shapes::Sphere(shape) => shape.material,
            Shapes::Plane(shape) => shape.material,
            // Shapes::Cube(shape) => shape.material,
            // Shapes::Cylinder(shape) => shape.get_material(),
            // Shapes::Cone(shape) => shape.get_material(),
        }
    }
}

impl Shape for Shapes {
    fn id(&self) -> Uuid {
        match self {
            Shapes::Sphere(shape) => shape.id(),
            Shapes::Plane(shape) => shape.id(),
            // Shapes::Cube(shape) => shape.id(),
            // Shapes::Cylinder(shape) => shape.id(),
            // Shapes::Cone(shape) => shape.id(),
        }
    }

    fn get_material(&self) -> Material {
        match self {
            Shapes::Sphere(shape) => shape.get_material(),
            Shapes::Plane(shape) => shape.get_material(),
            // Shapes::Cube(shape) => shape.get_material(),
            // Shapes::Cylinder(shape) => shape.get_material(),
            // Shapes::Cone(shape) => shape.get_material(),
        }
    }

    // fn set_material(&mut self, material: Material) -> Self {
    //     match self {
    //         Shapes::Sphere(shape) => shape.set_material(material).into(),
    //         Shapes::Plane(shape) => shape.set_material(material).into(),
    //         // Shapes::Cube(shape) => shape.set_material(material).into(),
    //         // Shapes::Cylinder(shape) => shape.set_material(material).into(),
    //         // Shapes::Cone(shape) => shape.set_material(material).into(),
    //     }
    // }

    fn get_transform(&self) -> crate::matrix::Matrix<4> {
        match self {
            Shapes::Sphere(shape) => shape.get_transform(),
            Shapes::Plane(shape) => shape.get_transform(),
            // Shapes::Cube(shape) => shape.get_transform(),
            // Shapes::Cylinder(shape) => shape.get_transform(),
            // Shapes::Cone(shape) => shape.get_transform(),
        }
    }

    // fn set_transform(&mut self, transform: crate::matrix::Matrix<4>) -> Self {
    //     match self {
    //         Shapes::Sphere(shape) => shape.set_transform(transform).into(),
    //         Shapes::Plane(shape) => shape.set_transform(transform).into(),
    //         // Shapes::Cube(shape) => shape.set_transform(transform).into(),
    //         // Shapes::Cylinder(shape) => shape.set_transform(transform).into(),
    //         // Shapes::Cone(shape) => shape.set_transform(transform).into(),
    //     }
    // }

    fn intersection(&self, t: f64) -> Intersection {
        match self {
            Shapes::Sphere(shape) => shape.intersection(t),
            Shapes::Plane(shape) => shape.intersection(t),
            // Shapes::Cube(shape) => shape.intersection(t),
            // Shapes::Cylinder(shape) => shape.intersection(t),
            // Shapes::Cone(shape) => shape.intersection(t),
        }
    }

    fn local_intersect(&self, local_ray: &Ray) -> Option<Vec<Intersection>> {
        match self {
            Shapes::Sphere(shape) => shape.local_intersect(local_ray),
            Shapes::Plane(shape) => shape.local_intersect(local_ray),
            // Shapes::Cube(shape) => shape.local_intersect(local_ray),
            // Shapes::Cylinder(shape) => shape.local_intersect(local_ray),
            // Shapes::Cone(shape) => shape.local_intersect(local_ray),
        }
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        match self {
            Shapes::Sphere(shape) => shape.local_normal_at(local_point),
            Shapes::Plane(shape) => shape.local_normal_at(local_point),
            // Shapes::Cube(shape) => shape.local_normal_at(local_point),
            // Shapes::Cylinder(shape) => shape.local_normal_at(local_point),
            // Shapes::Cone(shape) => shape.local_normal_at(local_point),
        }
    }
}

impl From<Sphere> for Shapes {
    fn from(shape: Sphere) -> Self {
        Shapes::Sphere(shape)
    }
}

impl From<Plane> for Shapes {
    fn from(shape: Plane) -> Self {
        Shapes::Plane(shape)
    }
}

// impl From<Cube> for Shapes {
//     fn from(shape: Cube) -> Self {
//         Shapes::Cube(shape)
//     }
// }

// impl From<Cylinder> for Shapes {
//     fn from(shape: Cylinder) -> Self {
//         Shapes::Cylinder(shape)
//     }
// }

// impl From<Cone> for Shapes {
//     fn from(shape: Cone) -> Self {
//         Shapes::Cone(shape)
//     }
// }
