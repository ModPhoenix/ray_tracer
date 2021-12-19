use std::f64::consts::PI;

use ray_tracer::{
    camera::Camera,
    color::Color,
    light::Light,
    material::Material,
    matrix::Matrix,
    patterns::{checkers::Checkers, gradient::Gradient, Pattern},
    shapes::{cube::Cube, plane::Plane, sphere::Sphere, Shape},
    tuple::Tuple,
    world::World,
};

fn main() -> std::io::Result<()> {
    let floor_material = Material::default()
        .set_color(Color::new(1., 0.9, 0.9))
        .set_specular(0.)
        .set_reflective(0.2)
        .set_pattern(Checkers::new(Color::new(0.2, 0.2, 0.2), Color::new(0.5, 0.5, 0.5)).into());

    let floor = Plane::default().set_material(floor_material);

    let cube = Cube::default()
        .set_material(
            Material::default()
                .set_color(Color::new(0.1, 0., 0.))
                .set_transparency(1.)
                .set_refractive_index(1.5)
                .set_ambient(0.1)
                .set_diffuse(0.1)
                .set_specular(1.)
                .set_shininess(300.),
        )
        .set_transform(
            Matrix::identity()
                .scaling(0.5, 0.5, 0.5)
                .translation(-0.5, 0.5, -1.5),
        );

    let cube2 = Cube::default()
        .set_material(
            Material::default()
                .set_pattern(Checkers::new(Color::new(1., 1., 0.), Color::new(0., 1., 1.)).into()),
        )
        .set_transform(
            Matrix::identity()
                .scaling(0.3, 0.3, 0.3)
                .rotation_y(PI / 4.)
                .translation(1.5, 0.3, -1.5),
        );

    let sky = Plane::default()
        .set_material(
            Material::default()
                .set_color(Color::new(0.6, 0.6, 0.9))
                .set_refractive_index(1.)
                .set_specular(0.9),
        )
        .set_transform(Matrix::identity().translation(0., 100., 0.));

    let middle = Sphere::default()
        .set_material(
            Material::default()
                .set_color(Color::new(0.1, 1., 0.5))
                .set_reflective(0.5)
                .set_refractive_index(1.5)
                .set_pattern(Checkers::new(Color::new(0., 1., 0.), Color::new(1., 0., 1.)).into()),
        )
        .set_transform(Matrix::identity().translation(-0.5, 1., 1.5));

    let middle2 = Sphere::default()
        .set_material(
            Material::default()
                .set_color(Color::new(0., 0.1, 0.))
                .set_transparency(1.)
                .set_refractive_index(1.5)
                .set_ambient(0.1)
                .set_diffuse(0.1)
                .set_specular(0.1)
                .set_shininess(300.),
        )
        .set_transform(
            Matrix::identity()
                .scaling(0.5, 0.5, 0.5)
                .translation(0., 0.5, -0.5),
        );

    let right = Sphere::default()
        .set_material(
            Material::default()
                .set_color(Color::new(0., 0., 0.2))
                .set_transparency(0.8)
                .set_refractive_index(1.5)
                .set_ambient(0.1)
                .set_diffuse(0.1)
                .set_specular(1.)
                .set_shininess(300.),
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
                .set_pattern(
                    Gradient::new(Color::new(1., 1., 0.), Color::new(0., 1., 0.))
                        .set_transform(Matrix::identity().rotation_y(PI / 2.).rotation_x(PI / 2.))
                        .into(),
                ),
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
        vec![
            floor.into(),
            sky.into(),
            middle.into(),
            middle2.into(),
            cube.into(),
            cube2.into(),
            right.into(),
            left.into(),
        ],
    );

    // 4K - 3840 × 2160
    // 8K - 7680 × 4320

    let camera = Camera::new(1000, 500, PI / 3.).set_transform(Matrix::identity().view_transform(
        Tuple::point(0., 2., -7.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);

    let img = image::load_from_memory(&canvas.to_ppm().as_bytes()).unwrap();

    img.save("scene.png").unwrap();

    Ok(())
}
