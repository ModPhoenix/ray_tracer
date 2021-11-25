use std::ops::Index;

use crate::ray::Ray;

pub trait Intersectable<T>
where
    T: Intersectable<T>,
{
    fn intersection(&self, t: f64) -> Intersection<T>;

    fn intersections(intersections: Vec<Intersection<T>>) -> Intersections<T> {
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
        let s = Sphere::new();
        let i = s.intersection(3.5);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = s.intersection(1.0);
        let i2 = s.intersection(2.0);
        let xs = Sphere::intersections(vec![i1, i2]);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[1].t, 2.);
    }
}
