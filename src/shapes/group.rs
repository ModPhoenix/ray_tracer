use uuid::Uuid;

use crate::{
    intersections::Intersection, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple,
};

use super::Shape;

#[derive(Debug, PartialEq)]
pub struct Group {
    id: Uuid,
    transform: Matrix<4>,
    material: Material,
    pub objects: Vec<Box<dyn Shape>>,
}

impl Group {
    pub fn new(transform: Matrix<4>, material: Material, objects: Vec<Box<dyn Shape>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform,
            material,
            objects,
        }
    }

    pub fn add_object(&mut self, mut shape: Box<dyn Shape>) {
        // shape.set_parent_id(self.id);
        // self.objects.push(shape);
        todo!()
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }
}

impl Default for Group {
    fn default() -> Self {
        Group::new(Matrix::identity(), Material::default(), vec![])
    }
}

impl Shape for Group {
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
        todo!()
    }

    fn local_intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        todo!()
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        matrix::Matrix,
        shapes::{sphere::Sphere, Shape},
    };

    use super::Group;

    // Scenario: Creating a new group
    //   Given g ← group()
    //   Then g.transform = identity_matrix
    //     And g is empty

    #[test]
    fn creating_a_new_group() {
        let group = Group::default();

        assert_eq!(group.get_transform(), Matrix::identity());
    }

    // Scenario: Adding a child to a group
    //   Given g ← group()
    //     And s ← test_shape()
    //   When add_child(g, s)
    //   Then g is not empty
    //     And g includes s
    //     And s.parent = g

    // #[test]
    // fn adding_a_child_to_a_group() {
    //     let mut g = Group::default();
    //     let mut s = Sphere::default();
    //     s.set_parent_id(g.id);
    //     g.add_object(Box::new(s));

    //     assert!(!g.objects.is_empty());
    //     assert_eq!(g.objects[0].parent_id().unwrap(), g.id());
    // }
}
