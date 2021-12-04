use std::ops::Index;

use crate::{object::Object, ray::Ray, tuple::Tuple, world::Normal};

pub trait Intersectable {
    fn intersection(&self, t: f64) -> Intersection;

    fn intersections(intersections: Vec<Intersection>) -> Intersections {
        Intersections::new(intersections)
    }

    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>>;
}

#[derive(Debug, Clone)]
pub struct ComputedIntersection {
    pub t: f64,
    pub object: Object,
    pub point: Tuple,
    pub normalv: Tuple,
    pub eyev: Tuple,
    pub inside: bool,
}

impl ComputedIntersection {
    pub fn new(
        t: f64,
        object: Object,
        point: Tuple,
        normalv: Tuple,
        eyev: Tuple,
        inside: bool,
    ) -> Self {
        Self {
            t,
            object,
            point,
            normalv,
            eyev,
            inside,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Object,
}

impl Intersection {
    pub fn new(t: f64, object: Object) -> Self {
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

        ComputedIntersection::new(self.t, self.object.clone(), point, normalv, eyev, inside)
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
    use crate::{intersections::Intersectable, ray::Ray, sphere::Sphere, tuple::Tuple};

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
        let xs = Sphere::intersections(vec![i1, i2]);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[1].t, 2.);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = s.intersection(1.0);
        let i2 = s.intersection(2.0);
        let i = Sphere::intersections(vec![i2, i1.clone()]);

        assert_eq!(i.hit(), Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = s.intersection(-1.0);
        let i2 = s.intersection(1.0);
        let i = Sphere::intersections(vec![i2.clone(), i1]);

        assert_eq!(i.hit(), Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = s.intersection(-2.0);
        let i2 = s.intersection(-1.0);
        let i = Sphere::intersections(vec![i2, i1]);

        assert_eq!(i.hit(), None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = s.intersection(5.0);
        let i2 = s.intersection(7.0);
        let i3 = s.intersection(-3.0);
        let i4 = s.intersection(2.0);
        let i = Sphere::intersections(vec![i1, i2, i3, i4.clone()]);

        assert_eq!(i.hit(), Some(i4));
    }
}
