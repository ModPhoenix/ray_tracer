use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

use ray_tracer::camera::Camera;
use ray_tracer::light::Light;
use ray_tracer::material::Material;
use ray_tracer::matrix::Matrix;
use ray_tracer::patterns::stripe::Stripe;
use ray_tracer::patterns::Pattern;
use ray_tracer::shapes::{plane::Plane, sphere::Sphere, Shape};
use ray_tracer::world::World;
use ray_tracer::{color::Color, tuple::Tuple};

fn main() -> std::io::Result<()> {
    let walls_material = Material::default()
        .set_color(Color::new(1., 0.9, 0.9))
        .set_specular(0.)
        .set_pattern(
            Stripe::new(Color::new_white(), Color::new(1., 0., 0.))
                .set_transform(Matrix::identity().scaling(0.5, 0.1, 0.1))
                .into(),
        );

    let floor = Plane::default().set_material(walls_material);

    let middle = Sphere::default()
        .set_material(
            Material::default()
                .set_color(Color::new(0.1, 1., 0.5))
                .set_diffuse(0.9)
                .set_specular(0.1)
                .set_pattern(
                    Stripe::new(Color::new_white(), Color::new(0., 0.8, 0.))
                        .set_transform(
                            Matrix::identity()
                                .rotation_y(PI / 2.)
                                .rotation_x(PI / 2.)
                                .scaling(0.1, 0.1, 0.1),
                        )
                        .into(),
                ),
        )
        .set_transform(Matrix::identity().translation(-0.5, 1., 1.5));

    let right = Sphere::default()
        .set_material(
            Material::default()
                .set_color(Color::new(1., 0., 0.))
                .set_diffuse(0.7)
                .set_specular(0.3)
                .set_pattern(
                    Stripe::new(Color::new(0., 1., 1.), Color::new(1., 1., 0.))
                        .set_transform(Matrix::identity().rotation_y(PI / 2.).rotation_x(PI / 2.))
                        .into(),
                ),
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
                .set_specular(0.3),
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

    let camera = Camera::new(1500, 1000, PI / 3.).set_transform(Matrix::identity().view_transform(
        Tuple::point(0., 2., -10.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
