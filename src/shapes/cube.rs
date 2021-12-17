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

    fn local_intersect(&self, ray: &crate::ray::Ray) -> Option<Vec<Intersection>> {
        let (xtmin, xtmax) = Cube::check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = Cube::check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = Cube::check_axis(ray.origin.z, ray.direction.z);

        let tmin_arr = [xtmin, ytmin, ztmin];
        let tmax_arr = [xtmax, ytmax, ztmax];

        let tmin = tmin_arr.iter().max_by(|a, b| a.partial_cmp(b).unwrap());
        let tmax = tmax_arr.iter().min_by(|a, b| a.partial_cmp(b).unwrap());

        Some(vec![
            self.intersection(*tmin.unwrap()),
            self.intersection(*tmax.unwrap()),
        ])
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        Tuple::vector(0., 1., 0.)
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

            assert_eq!(xs.clone().unwrap().len(), 2);
            assert_eq!(xs.clone().unwrap()[0].t, t1);
            assert_eq!(xs.clone().unwrap()[1].t, t2);
        }
    }
}
