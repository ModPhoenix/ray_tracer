use crate::{canvas::Canvas, matrix::Matrix, ray::Ray, tuple::Tuple, world::World};

#[derive(Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    transform: Matrix<4>,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.).tan();
        let aspect = hsize as f64 / vsize as f64;

        let half_width: f64;
        let half_height: f64;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.) / hsize as f64;

        Self {
            hsize,
            vsize,
            field_of_view,
            half_width,
            half_height,
            pixel_size,
            transform: Matrix::identity(),
        }
    }

    /// Get a reference to the camera's hsize.
    pub fn hsize(&self) -> usize {
        self.hsize
    }

    /// Get a reference to the camera's vsize.
    pub fn vsize(&self) -> usize {
        self.vsize
    }

    pub fn set_transform(mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inverse_transform = self.transform.inverse();

        let pixel = inverse_transform * Tuple::point(world_x, world_y, -1.);
        let origin = inverse_transform * Tuple::point(0., 0., 0.);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, 5);

                image.set(x, y, &color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::color::Color;
    use crate::light::Light;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::shapes::sphere::Sphere;
    use crate::shapes::Shape;
    use crate::tuple::Tuple;
    use crate::utils::fuzzy_equal::fuzzy_equal;
    use crate::world::World;

    use super::Camera;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.);
        assert_eq!(c.transform, Matrix::identity());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.);

        assert!(fuzzy_equal(c.pixel_size, 0.01));
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.);

        assert!(fuzzy_equal(c.pixel_size, 0.01));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0., 0., 0.));
        assert_eq!(r.direction, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Tuple::point(0., 0., 0.));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let c = Camera::new(201, 101, PI / 2.).set_transform(
            Matrix::identity()
                .translation(0., -2., 5.)
                .rotation_y(PI / 4.),
        );

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0., 2., -5.));
        assert_eq!(
            r.direction,
            Tuple::vector(2.0_f64.sqrt() / 2., 0., -2.0_f64.sqrt() / 2.)
        );
    }

    fn default_world() -> World {
        let light = Light::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
        let s1 = Sphere::default().set_material(
            Material::default()
                .set_color(Color::new(0.8, 1.0, 0.6))
                .set_diffuse(0.7)
                .set_specular(0.2),
        );
        let s2 = Sphere::default().set_transform(Matrix::identity().scaling(0.5, 0.5, 0.5));

        World::new(Some(light), vec![s1.into(), s2.into()])
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = default_world();

        let from = Tuple::point(0., 0., -5.);
        let to = Tuple::point(0., 0., 0.);
        let up = Tuple::vector(0., 1., 0.);
        let c = Camera::new(11, 11, PI / 2.)
            .set_transform(Matrix::identity().view_transform(from, to, up));

        let image = c.render(w);

        assert_eq!(image.get(5, 5), &Color::new(0.38066, 0.47583, 0.2855));
    }
}
