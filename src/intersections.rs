use std::ops::Index;

use crate::{
    constants::EPSILON,
    ray::Ray,
    shapes::{Shape, Shapes},
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
    pub reflectv: Tuple,
    pub inside: bool,
    pub n1: f64,
    pub n2: f64,
}

impl ComputedIntersection {
    pub fn new(
        t: f64,
        object: Shapes,
        point: Tuple,
        over_point: Tuple,
        normalv: Tuple,
        eyev: Tuple,
        reflectv: Tuple,
        inside: bool,
        n1: f64,
        n2: f64,
    ) -> Self {
        Self {
            t,
            object,
            point,
            over_point,
            normalv,
            eyev,
            reflectv,
            inside,
            n1,
            n2,
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

    pub fn prepare_computations(&self, ray: &Ray, xs: Intersections) -> ComputedIntersection {
        let point = ray.position(self.t);
        let mut normalv = self.object.normal_at(point);
        let eyev = -ray.direction;
        let inside;
        let mut n1 = 0.;
        let mut n2 = 0.;

        if Tuple::dot(&normalv, &eyev) < 0. {
            inside = true;
            normalv = -normalv;
        } else {
            inside = false;
        }

        let over_point = point + normalv * EPSILON;
        let reflectv = ray.direction.reflect(normalv);

        let mut containers: Vec<Shapes> = vec![];

        for i in xs.into_iter() {
            if Some(&i) == Some(self) {
                if containers.is_empty() {
                    n1 = 1.;
                } else {
                    n1 = containers
                        .last()
                        .unwrap()
                        .get_material()
                        .get_refractive_index();
                }
            }

            if containers.contains(&i.object) {
                containers = containers
                    .into_iter()
                    .filter(|item| item != &i.object)
                    .collect();
            } else {
                containers.push(i.object.clone())
            }

            if Some(&i) == Some(self) {
                if containers.is_empty() {
                    n2 = 1.;
                } else {
                    n2 = containers
                        .last()
                        .unwrap()
                        .get_material()
                        .get_refractive_index();
                }

                break;
            }
        }

        ComputedIntersection::new(
            self.t,
            self.object.clone(),
            point,
            over_point,
            normalv,
            eyev,
            reflectv,
            inside,
            n1,
            n2,
        )
    }
}

#[derive(Debug, Clone)]
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

    pub fn into_iter(self) -> std::vec::IntoIter<Intersection> {
        self.data.into_iter()
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

impl Default for Intersections {
    fn default() -> Self {
        Self::new(vec![])
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
        constants::EPSILON,
        intersections::Intersections,
        material::Material,
        matrix::Matrix,
        ray::Ray,
        shapes::{plane::Plane, sphere::Sphere, Shape},
        tuple::Tuple,
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

        let comps = i.prepare_computations(&r, Intersections::default());

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Tuple::point(0., 0., -1.));
        assert_eq!(comps.eyev, Tuple::vector(0., 0., -1.));
        assert_eq!(comps.normalv, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Plane::default();
        let r = Ray::new(
            Tuple::point(0., 1., -1.),
            Tuple::vector(0., -2.0_f64.sqrt() / 2., 2.0_f64.sqrt() / 2.),
        );

        let i = shape.intersection(2.0_f64.sqrt());
        let comps = i.prepare_computations(&r, Intersections::default());

        assert_eq!(
            comps.reflectv,
            Tuple::vector(0., 2.0_f64.sqrt() / 2., 2.0_f64.sqrt() / 2.)
        );
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = shape.intersection(4.);

        let comps = i.prepare_computations(&r, Intersections::default());

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = shape.intersection(1.);

        let comps = i.prepare_computations(&r, Intersections::default());

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
        let comps = i.prepare_computations(&r, Intersections::default());

        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = Sphere::new_glass()
            .set_transform(Matrix::identity().scaling(2., 2., 2.))
            .set_material(Material::default().set_refractive_index(1.5));
        let b = Sphere::new_glass()
            .set_transform(Matrix::identity().translation(0., 0., -0.25))
            .set_material(Material::default().set_refractive_index(2.));
        let c = Sphere::new_glass()
            .set_transform(Matrix::identity().translation(0., 0., 0.25))
            .set_material(Material::default().set_refractive_index(2.5));

        let r = Ray::new(Tuple::point(0., 0., -4.), Tuple::vector(0., 0., 1.));
        let xs = Intersections::new(vec![
            a.intersection(2.),
            b.intersection(2.75),
            c.intersection(3.25),
            b.intersection(4.75),
            c.intersection(5.25),
            a.intersection(6.),
        ]);

        let examples = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for (index, (n1, n2)) in examples.into_iter().enumerate() {
            let comps = xs[index].prepare_computations(&r, xs.clone());

            assert_eq!(comps.n1, n1);
            assert_eq!(comps.n2, n2);
        }
    }
}
