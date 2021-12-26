use std::mem::swap;

use uuid::Uuid;

use crate::{
    intersections::Intersection, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple,
};

use super::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Cylinder {
    id: Uuid,
    transform: Matrix<4>,
    material: Material,
    minimum: f64,
    maximum: f64,
}

impl Cylinder {
    pub fn new(transform: Matrix<4>, material: Material, minimum: f64, maximum: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform,
            material,
            minimum,
            maximum,
        }
    }

    /// Get a reference to the cylinder's minimum.
    pub fn minimum(&self) -> f64 {
        self.minimum
    }

    /// Set the cylinder's minimum.
    pub fn set_minimum(&mut self, minimum: f64) -> Self {
        self.minimum = minimum;
        self.clone()
    }

    /// Get a reference to the cylinder's maximum.
    pub fn maximum(&self) -> f64 {
        self.maximum
    }

    /// Set the cylinder's maximum.
    pub fn set_maximum(&mut self, maximum: f64) -> Self {
        self.maximum = maximum;
        self.clone()
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder::new(
            Matrix::identity(),
            Material::default(),
            f64::NEG_INFINITY,
            f64::INFINITY,
        )
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

        let mut t0 = (-b - disc.sqrt()) / (2. * a);
        let mut t1 = (-b + disc.sqrt()) / (2. * a);

        if t0 > t1 {
            swap(&mut t0, &mut t1);
        }

        let mut xs: Vec<Intersection> = vec![];

        let y0 = ray.origin.y + t0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(self.intersection(t0));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(self.intersection(t1));
        }

        Some(xs)
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        Tuple::vector(point.x, 0., point.z)
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

    #[test]
    fn normal_vector_on_a_cylinder() {
        let cyl = Cylinder::default();

        let examples = vec![
            (Tuple::point(1., 0., 0.), Tuple::vector(1., 0., 0.)),
            (Tuple::point(0., 5., -1.), Tuple::vector(0., 0., -1.)),
            (Tuple::point(0., -2., 1.), Tuple::vector(0., 0., 1.)),
            (Tuple::point(-1., 1., 0.), Tuple::vector(-1., 0., 0.)),
        ];

        for (point, normal) in examples.into_iter() {
            let n = cyl.local_normal_at(point);

            assert_eq!(n, normal);
        }
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::default();

        assert_eq!(cyl.minimum(), f64::NEG_INFINITY);
        assert_eq!(cyl.maximum(), f64::INFINITY);
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let cyl = Cylinder::default().set_minimum(1.).set_maximum(2.);

        let examples = vec![
            (Tuple::point(0., 1.5, 0.), Tuple::vector(0.1, 1., 0.), 0),
            (Tuple::point(0., 3., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 1.5, -2.), Tuple::vector(0., 0., 1.), 2),
        ];

        for (point, direction, count) in examples.into_iter() {
            let direction = direction.normalize();
            let r = Ray::new(point, direction);

            let xs = cyl.local_intersect(&r);

            assert_eq!(xs.unwrap_or(vec![]).len(), count);
        }
    }
}
