use std::rc::Rc;

use uuid::Uuid;

use crate::{
    constants::EPSILON, intersections::Intersection, material::Material, matrix::Matrix,
    tuple::Tuple,
};

use super::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Cube {
    id: Uuid,
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Cube {
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

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1. - origin;
        let tmax_numerator = 1. - origin;

        let mut tmin;
        let mut tmax;

        if direction.abs() >= EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * f64::INFINITY;
            tmax = tmax_numerator * f64::INFINITY;
        }

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        (tmin, tmax)
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube::new(Matrix::identity(), Material::default())
    }
}

impl Shape for Cube {
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
        Intersection::new(t, Rc::new(self.clone()))
    }

    fn local_intersect(&self, ray: &crate::ray::Ray) -> Option<Vec<Intersection>> {
        let (xtmin, xtmax) = Cube::check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = Cube::check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = Cube::check_axis(ray.origin.z, ray.direction.z);

        let tmin_arr = [xtmin, ytmin, ztmin];
        let tmax_arr = [xtmax, ytmax, ztmax];

        let tmin = tmin_arr.iter().max_by(|a, b| a.partial_cmp(b).unwrap());
        let tmax = tmax_arr.iter().min_by(|a, b| a.partial_cmp(b).unwrap());

        if tmin > tmax {
            None
        } else {
            Some(vec![
                self.intersection(*tmin.unwrap()),
                self.intersection(*tmax.unwrap()),
            ])
        }
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        let maxc_arr = [point.x.abs(), point.y.abs(), point.z.abs()];
        let maxc = maxc_arr
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        if maxc == &point.x.abs() {
            return Tuple::vector(point.x, 0., 0.);
        } else if maxc == &point.y.abs() {
            return Tuple::vector(0., point.y, 0.);
        }

        Tuple::vector(0., 0., point.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ray::Ray,
        shapes::{cube::Cube, Shape},
        tuple::Tuple,
    };

    #[test]
    fn a_ray_intersects_a_cube() {
        let c = Cube::default();

        #[rustfmt::skip]
        let examples = vec![
            (Tuple::point( 5., 0.5,  0.), Tuple::vector(-1.,  0.,  0.),  4., 6.),
            (Tuple::point(-5., 0.5,  0.), Tuple::vector( 1.,  0.,  0.),  4., 6.),
            (Tuple::point(0.5,  5.,  0.), Tuple::vector( 0., -1.,  0.),  4., 6.),
            (Tuple::point(0.5, -5.,  0.), Tuple::vector( 0.,  1.,  0.),  4., 6.),
            (Tuple::point(0.5,  0.,  5.), Tuple::vector( 0.,  0., -1.),  4., 6.),
            (Tuple::point(0.5,  0., -5.), Tuple::vector( 0.,  0.,  1.),  4., 6.),
            (Tuple::point( 0., 0.5,  0.), Tuple::vector( 0.,  0.,  1.), -1., 1.),
        ];

        for (origin, direction, t1, t2) in examples.into_iter() {
            let r = Ray::new(origin, direction);
            let xs = c.local_intersect(&r);

            assert_eq!(xs.as_ref().unwrap().len(), 2);
            assert_eq!(xs.as_ref().unwrap()[0].t, t1);
            assert_eq!(xs.unwrap()[1].t, t2);
        }
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let c = Cube::default();

        #[rustfmt::skip]
        let examples = vec![
            (Tuple::point(-2.,  0.,  0.), Tuple::vector(0.2673, 0.5345, 0.8018)),
            (Tuple::point( 0., -2.,  0.), Tuple::vector(0.8018, 0.2673, 0.5345)),
            (Tuple::point( 0.,  0., -2.), Tuple::vector(0.5345, 0.8018, 0.2673)),
            (Tuple::point( 2.,  0.,  2.), Tuple::vector( 0.,  0., -1.)),
            (Tuple::point( 0.,  2.,  2.), Tuple::vector( 0., -1.,  0.)),
            (Tuple::point( 2.,  2.,  0.), Tuple::vector(-1.,  0.,  0.)),
        ];

        for (origin, direction) in examples.into_iter() {
            let r = Ray::new(origin, direction);
            let xs = c.local_intersect(&r);

            assert!(xs.is_none());
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = Cube::default();

        #[rustfmt::skip]
        let examples = vec![
            (Tuple::point(  1.,  0.5, -0.8), Tuple::vector( 1.,  0.,  0.)),
            (Tuple::point( -1., -0.2,  0.9), Tuple::vector(-1.,  0.,  0.)),
            (Tuple::point(-0.4,   1., -0.1), Tuple::vector( 0.,  1.,  0.)),
            (Tuple::point( 0.3,  -1., -0.7), Tuple::vector( 0., -1.,  0.)),
            (Tuple::point(-0.6,  0.3,  1.0), Tuple::vector( 0.,  0.,  1.)),
            (Tuple::point( 0.4,  0.4, -1.0), Tuple::vector( 0.,  0., -1.)),
            (Tuple::point(  1.,   1.,  1.0), Tuple::vector( 1.,  0.,  0.)),
            (Tuple::point( -1.,  -1., -1.0), Tuple::vector(-1.,  0.,  0.)),
        ];

        for (point, normal) in examples.into_iter() {
            let c_normal = c.local_normal_at(point);

            assert_eq!(c_normal, normal);
        }
    }
}
