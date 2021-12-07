use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

use ray_tracer::camera::Camera;
use ray_tracer::light::Light;
use ray_tracer::material::Material;
use ray_tracer::matrix::Matrix;
use ray_tracer::pattern::Pattern;
use ray_tracer::plane::Plane;
use ray_tracer::shape::Shape;
use ray_tracer::sphere::Sphere;
use ray_tracer::world::World;
use ray_tracer::{color::Color, tuple::Tuple};

fn main() -> std::io::Result<()> {
    let walls_material = Material::default()
        .set_color(Color::new(1., 0.9, 0.9))
        .set_specular(0.)
        .set_pattern(Pattern::stripe_pattern(
            Color::new_white(),
            Color::new(0.7, 0., 0.),
        ));

    let floor = Plane::default().set_material(walls_material);

    let middle = Sphere::default()
        .set_material(
            Material::default()
                .set_color(Color::new(0.1, 1., 0.5))
                .set_diffuse(0.9)
                .set_specular(0.1)
                .set_pattern(Pattern::stripe_pattern(
                    Color::new_white(),
                    Color::new(0.7, 0., 0.),
                )),
        )
        .set_transform(Matrix::identity().translation(-0.5, 1., 1.5));

    let right = Sphere::default()
        .set_material(
            Material::default()
                .set_color(Color::new(1., 0., 0.))
                .set_diffuse(0.7)
                .set_specular(0.3)
                .set_pattern(Pattern::stripe_pattern(
                    Color::new_white(),
                    Color::new(0.7, 0., 0.),
                )),
        )
        .set_transform(
            Matrix::identity()
                .scaling(0.5, 0.5, 0.5)
                .translation(1.5, 0.5, -0.5),
        );

    let left = Sphere::default()
        .set_material(
            Material::default()
                .set_color(Color::new(1., 0., 1.))
                .set_diffuse(0.7)
                .set_specular(0.3)
                .set_pattern(Pattern::stripe_pattern(
                    Color::new_white(),
                    Color::new(0.7, 0., 0.),
                )),
        )
        .set_transform(
            Matrix::identity()
                .scaling(0.33, 0.33, 0.33)
                .translation(-1.5, 0.33, -0.75),
        );

    let world = World::new(
        Some(Light::new(
            Tuple::point(-10., 10., -10.),
            Color::new(1., 1., 1.),
        )),
        vec![floor.into(), middle.into(), right.into(), left.into()],
    );

    // 4K - 4096 × 3112
    // 8K - 7680 × 4320

    let camera =
        Camera::new(1500, 1000, PI / 3.5).set_transform(Matrix::identity().view_transform(
            Tuple::point(0., 10., -10.),
            Tuple::point(0., 1., 0.),
            Tuple::vector(0., 1., 0.),
        ));

    let canvas = camera.render(world);

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
