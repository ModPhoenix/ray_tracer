use crate::{
    intersections::Intersection, material::Material, matrix::Matrix, ray::Ray, sphere::Sphere,
    tuple::Tuple,
};

pub trait Shape {
    // materials
    fn get_material(&self) -> Material;
    fn set_material(&mut self, material: Material) -> Self;

    // materials
    fn get_transform(&self) -> Matrix<4>;
    fn set_transform(&mut self, transform: Matrix<4>) -> Self;

    // intersection
    fn intersection(&self, t: f64) -> Intersection;
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let ray_transformed = ray.transform(self.get_transform().inverse());

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

            return Some(vec![self.intersection(t1), self.intersection(t2)]);
        }
    }

    // normal
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.get_transform().inverse() * world_point;
        let object_normal = object_point - Tuple::point(0., 0., 0.);
        let mut world_normal = self.get_transform().inverse().transpose() * object_normal;

        world_normal.w = 0.;

        world_normal.normalize()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Shapes {
    Sphere(Sphere),
}

impl Shapes {
    pub fn get_material(self) -> Material {
        match self {
            Shapes::Sphere(sphere) => sphere.material,
        }
    }
}

impl Shape for Shapes {
    fn get_material(&self) -> Material {
        match self {
            Shapes::Sphere(sphere) => sphere.get_material(),
        }
    }

    fn set_material(&mut self, material: Material) -> Self {
        match self {
            Shapes::Sphere(sphere) => sphere.set_material(material).into(),
        }
    }

    fn get_transform(&self) -> crate::matrix::Matrix<4> {
        match self {
            Shapes::Sphere(sphere) => sphere.get_transform(),
        }
    }

    fn set_transform(&mut self, transform: crate::matrix::Matrix<4>) -> Self {
        match self {
            Shapes::Sphere(sphere) => sphere.set_transform(transform).into(),
        }
    }

    fn intersection(&self, t: f64) -> Intersection {
        match self {
            Shapes::Sphere(sphere) => sphere.intersection(t),
        }
    }
}

impl From<Sphere> for Shapes {
    fn from(sphere: Sphere) -> Self {
        Shapes::Sphere(sphere)
    }
}
