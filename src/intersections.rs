use std::ops::Index;

use crate::{
    constants::EPSILON,
    ray::Ray,
    shape::{Shape, Shapes},
    tuple::Tuple,
};

#[derive(Debug, Clone)]
pub struct ComputedIntersection {
    pub t: f64,
    pub object: Shapes,
    pub point: Tuple,
    pub over_point: Tuple,
    pub normalv: Tuple,
    pub eyev: Tuple,
    pub inside: bool,
}

impl ComputedIntersection {
    pub fn new(
        t: f64,
        object: Shapes,
        point: Tuple,
        over_point: Tuple,
        normalv: Tuple,
        eyev: Tuple,
        inside: bool,
    ) -> Self {
        Self {
            t,
            object,
            point,
            over_point,
            normalv,
            eyev,
            inside,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Shapes,
}

impl Intersection {
    pub fn new(t: f64, object: Shapes) -> Self {
        Self { t, object }
    }

    pub fn prepare_computations(&self, ray: &Ray) -> ComputedIntersection {
        let point = ray.position(self.t);
        let mut normalv = self.object.normal_at(point);
        let eyev = -ray.direction;
        let inside;

        if Tuple::dot(&normalv, &eyev) < 0. {
            inside = true;
            normalv = -normalv;
        } else {
            inside = false;
        }

        let over_point = point + normalv * EPSILON;

        ComputedIntersection::new(
            self.t,
            self.object.clone(),
            point,
            over_point,
            normalv,
            eyev,
            inside,
        )
    }
}

#[derive(Debug)]
pub struct Intersections {
    data: Vec<Intersection>,
}

impl Intersections {
    pub fn new(mut intersections: Vec<Intersection>) -> Self {
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Self {
            data: intersections,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn hit(self) -> Option<Intersection> {
        for intersection in self.data.into_iter() {
            if intersection.t > 0.0 {
                return Some(intersection);
            }
        }

        None
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::EPSILON, intersections::Intersections, matrix::Matrix, ray::Ray, shape::Shape,
        sphere::Sphere, tuple::Tuple,
    };

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::default();
        let i = s.intersection(3.5);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s.into());
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = shape.intersection(4.);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Tuple::point(0., 0., -1.));
        assert_eq!(comps.eyev, Tuple::vector(0., 0., -1.));
        assert_eq!(comps.normalv, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = shape.intersection(4.);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = shape.intersection(1.);

        let comps = i.prepare_computations(&r);

        assert_eq!(comps.point, Tuple::point(0., 0., 1.));
        assert_eq!(comps.eyev, Tuple::vector(0., 0., -1.));
        assert_eq!(comps.inside, true);
        // normal would have been (0, 0, 1), but is inverted!
        assert_eq!(comps.normalv, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();
        let i1 = s.intersection(1.0);
        let i2 = s.intersection(2.0);
        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[1].t, 2.);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = s.intersection(1.0);
        let i2 = s.intersection(2.0);
        let i = Intersections::new(vec![i2, i1.clone()]);

        assert_eq!(i.hit(), Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = s.intersection(-1.0);
        let i2 = s.intersection(1.0);
        let i = Intersections::new(vec![i2.clone(), i1]);

        assert_eq!(i.hit(), Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = s.intersection(-2.0);
        let i2 = s.intersection(-1.0);
        let i = Intersections::new(vec![i2, i1]);

        assert_eq!(i.hit(), None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = s.intersection(5.0);
        let i2 = s.intersection(7.0);
        let i3 = s.intersection(-3.0);
        let i4 = s.intersection(2.0);
        let i = Intersections::new(vec![i1, i2, i3, i4.clone()]);

        assert_eq!(i.hit(), Some(i4));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::default().set_transform(Matrix::identity().translation(0., 0., 1.));

        let i = shape.intersection(5.);
        let comps = i.prepare_computations(&r);

        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }
}
