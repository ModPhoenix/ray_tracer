use uuid::Uuid;

use crate::{
    intersections::Intersection, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple,
};

use super::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Cylinder {
    id: Uuid,
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Cylinder {
    pub fn new(transform: Matrix<4>, material: Material) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform,
            material,
        }
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder::new(Matrix::identity(), Material::default())
    }
}

impl Shape for Cylinder {
    fn id(&self) -> Uuid {
        self.id
    }

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

    fn local_intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let a = ray.direction.x.powf(2.) + ray.direction.z.powf(2.);
        if a <= 0. {
            return None;
        }

        let b = 2. * ray.origin.x * ray.direction.x + 2. * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powf(2.) + ray.origin.z.powf(2.) - 1.;
        let disc = b.powf(2.) - 4. * a * c;
        if disc < 0. {
            return None;
        }

        let t0 = (-b - disc.sqrt()) / (2. * a);
        let t1 = (-b + disc.sqrt()) / (2. * a);

        Some(vec![self.intersection(t0), self.intersection(t1)])
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ray::Ray, shapes::Shape, tuple::Tuple, utils::fuzzy_equal::fuzzy_equal};

    use super::Cylinder;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let cyl = Cylinder::default();

        let examples = vec![
            (Tuple::point(1., 0., 0.), Tuple::vector(0., 1., 0.)),
            (Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.)),
            (Tuple::point(0., 0., -5.), Tuple::vector(1., 1., 1.)),
        ];

        for (origin, direction) in examples.into_iter() {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);

            let xs = cyl.local_intersect(&r);

            assert_eq!(xs, None);
        }
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let cyl = Cylinder::default();

        let examples = vec![
            (Tuple::point(1., 0., -5.), Tuple::vector(0., 0., 1.), 5., 5.),
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.), 4., 6.),
            (
                Tuple::point(0.5, 0., -5.),
                Tuple::vector(0.1, 1., 1.),
                6.80798,
                7.08872,
            ),
        ];

        for (origin, direction, t0, t1) in examples.into_iter() {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);

            let xs = cyl.local_intersect(&r);

            assert_eq!(xs.as_ref().unwrap().len(), 2);
            assert!(fuzzy_equal(xs.as_ref().unwrap()[0].t, t0));
            assert!(fuzzy_equal(xs.as_ref().unwrap()[1].t, t1));
        }
    }
}
