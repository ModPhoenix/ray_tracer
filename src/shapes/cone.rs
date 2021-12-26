use std::mem::swap;

use uuid::Uuid;

use crate::{
    constants::EPSILON, intersections::Intersection, material::Material, matrix::Matrix, ray::Ray,
    tuple::Tuple, utils::fuzzy_equal::fuzzy_equal,
};

use super::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Cone {
    id: Uuid,
    transform: Matrix<4>,
    material: Material,
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Cone {
    pub fn new(
        transform: Matrix<4>,
        material: Material,
        minimum: f64,
        maximum: f64,
        closed: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform,
            material,
            minimum,
            maximum,
            closed,
        }
    }

    /// Get a reference to the cone's minimum.
    pub fn minimum(&self) -> f64 {
        self.minimum
    }

    /// Set the cone's minimum.
    pub fn set_minimum(&mut self, minimum: f64) -> Self {
        self.minimum = minimum;
        self.clone()
    }

    /// Get a reference to the cone's maximum.
    pub fn maximum(&self) -> f64 {
        self.maximum
    }

    /// Set the cone's maximum.
    pub fn set_maximum(&mut self, maximum: f64) -> Self {
        self.maximum = maximum;
        self.clone()
    }

    /// Get a reference to the cone's closed.
    pub fn closed(&self) -> bool {
        self.closed
    }

    /// Set the cone's closed.
    pub fn set_closed(&mut self, closed: bool) -> Self {
        self.closed = closed;
        self.clone()
    }

    pub fn intersect_caps(&self, ray: &Ray, xs: &mut Vec<Intersection>) {
        fn check_cap(ray: &Ray, t: f64, y: f64) -> bool {
            let x = ray.origin.x + t * ray.direction.x;
            let z = ray.origin.z + t * ray.direction.z;

            (x.powf(2.) + z.powf(2.)) <= y.powf(2.)
        }

        if !self.closed || fuzzy_equal(ray.direction.y, 0.) {
            return;
        }

        let t = (self.minimum - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t, self.minimum) {
            xs.push(self.intersection(t));
        }

        let t = (self.maximum - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t, self.maximum) {
            xs.push(self.intersection(t));
        }
    }
}

impl Default for Cone {
    fn default() -> Self {
        Cone::new(
            Matrix::identity(),
            Material::default(),
            f64::NEG_INFINITY,
            f64::INFINITY,
            false,
        )
    }
}

impl Shape for Cone {
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
        let mut xs: Vec<Intersection> = vec![];

        let a = ray.direction.x.powf(2.) - ray.direction.y.powf(2.) + ray.direction.z.powf(2.);
        let b = 2. * ray.origin.x * ray.direction.x - 2. * ray.origin.y * ray.direction.y
            + 2. * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powf(2.) - ray.origin.y.powf(2.) + ray.origin.z.powf(2.);

        if a == 0. && b != 0. {
            let t = -c / (2. * b);
            xs.push(self.intersection(t));
        } else if a.abs() >= 0. {
            let disc = b.powf(2.) - 4. * a * c;
            if disc < 0. {
                return None;
            }

            let mut t0 = (-b - disc.sqrt()) / (2. * a);
            let mut t1 = (-b + disc.sqrt()) / (2. * a);

            if t0 > t1 {
                swap(&mut t0, &mut t1);
            }

            let y0 = ray.origin.y + t0 * ray.direction.y;
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(self.intersection(t0));
            }

            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(self.intersection(t1));
            }
        }

        self.intersect_caps(ray, &mut xs);

        if xs.len() == 0 {
            None
        } else {
            Some(xs)
        }
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        let dist = point.x.powf(2.) + point.z.powf(2.);

        if dist < 1. && point.y >= self.maximum - EPSILON {
            return Tuple::vector(0., 1., 0.);
        } else if dist < 1. && point.y <= self.minimum + EPSILON {
            return Tuple::vector(0., -1., 0.);
        } else {
            let mut y = (point.x.powf(2.) + point.z.powf(2.)).sqrt();

            if point.y > 0. {
                y = -y;
            }

            return Tuple::vector(point.x, y, point.z);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        material::Material, matrix::Matrix, ray::Ray, shapes::Shape, tuple::Tuple,
        utils::fuzzy_equal::fuzzy_equal,
    };

    use super::Cone;

    #[test]
    fn the_default_cone() {
        let cone = Cone::default();

        assert_eq!(cone.get_transform(), Matrix::identity());
        assert_eq!(cone.get_material(), Material::default());
        assert_eq!(cone.minimum(), f64::NEG_INFINITY);
        assert_eq!(cone.maximum(), f64::INFINITY);
        assert_eq!(cone.closed(), false);
    }

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let cone = Cone::default();

        let examples = vec![
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.), 5., 5.),
            (
                Tuple::point(0., 0., -5.),
                Tuple::vector(1., 1., 1.),
                8.66025,
                8.66025,
            ),
            (
                Tuple::point(1., 1., -5.),
                Tuple::vector(-0.5, -1., 1.),
                4.55006,
                49.44994,
            ),
        ];

        for (origin, direction, t0, t1) in examples.into_iter() {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);

            let xs = cone.local_intersect(&r);

            assert_eq!(xs.as_ref().unwrap().len(), 2);
            assert!(fuzzy_equal(xs.as_ref().unwrap()[0].t, t0));
            assert!(fuzzy_equal(xs.as_ref().unwrap()[1].t, t1));
        }
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let cone = Cone::default();

        let direction = Tuple::vector(0., 1., 1.).normalize();
        let r = Ray::new(Tuple::point(0., 0., -1.), direction);

        let xs = cone.local_intersect(&r);

        assert_eq!(xs.as_ref().unwrap().len(), 1);
        assert!(fuzzy_equal(xs.as_ref().unwrap()[0].t, 0.35355));
    }

    #[test]
    fn intersecting_a_cones_end_caps() {
        let cone = Cone::default()
            .set_minimum(-0.5)
            .set_maximum(0.5)
            .set_closed(true);

        let examples = vec![
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.), 0),
            (Tuple::point(0., 0., -0.25), Tuple::vector(0., 1., 1.), 2),
            (Tuple::point(0., 0., -0.25), Tuple::vector(0., 1., 0.), 4),
        ];

        for (origin, direction, count) in examples.into_iter() {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);

            let xs = cone.local_intersect(&r);

            assert_eq!(xs.unwrap_or(vec![]).len(), count);
        }
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let cone = Cone::default()
            .set_minimum(-0.5)
            .set_maximum(0.5)
            .set_closed(true);

        let examples = vec![
            (Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 0.)),
            (
                Tuple::point(1., 1., 1.),
                Tuple::vector(1., -2.0_f64.sqrt(), 1.),
            ),
            (Tuple::point(-1., -1., 0.), Tuple::vector(-1., 1., 0.)),
        ];

        for (point, normal) in examples.into_iter() {
            let n = cone.local_normal_at(point);

            assert_eq!(n, normal);
        }
    }
}
