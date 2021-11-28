use std::ops::Index;

use crate::ray::Ray;

pub trait Intersectable<T>
where
    T: Intersectable<T>,
{
    fn intersection(&self, t: f64) -> Intersection<T>;

    fn intersections(mut intersections: Vec<Intersection<T>>) -> Intersections<T> {
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Intersections::new(intersections)
    }

    fn intersect(&self, ray: &Ray) -> Option<[Intersection<T>; 2]>;
}

#[derive(Debug, Clone, PartialEq)]

pub struct Intersection<T>
where
    T: Intersectable<T>,
{
    pub t: f64,
    pub object: T,
}

impl<T> Intersection<T>
where
    T: Intersectable<T>,
{
    pub fn new(t: f64, object: T) -> Self {
        Self { t, object }
    }
}

pub struct Intersections<T>
where
    T: Intersectable<T>,
{
    data: Vec<Intersection<T>>,
}

impl<T> Intersections<T>
where
    T: Intersectable<T>,
{
    pub fn new(intersections: Vec<Intersection<T>>) -> Self {
        Self {
            data: intersections,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn hit(&self) -> Option<&Intersection<T>> {
        for intersection in self.data.iter() {
            if intersection.t > 0.0 {
                return Some(intersection);
            }
        }

        None
    }
}

impl<T> Index<usize> for Intersections<T>
where
    T: Intersectable<T>,
{
    type Output = Intersection<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{intersections::Intersectable, sphere::Sphere};

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::default();
        let i = s.intersection(3.5);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
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

        assert_eq!(i.hit(), Some(&i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = s.intersection(-1.0);
        let i2 = s.intersection(1.0);
        let i = Sphere::intersections(vec![i2.clone(), i1]);

        assert_eq!(i.hit(), Some(&i2));
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

        assert_eq!(i.hit(), Some(&i4));
    }
}
