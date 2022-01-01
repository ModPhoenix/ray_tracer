use std::f64::consts::PI;

use ray_tracer::{
    camera::Camera,
    color::{Color, RGB},
    light::Light,
    material::Material,
    matrix::Matrix,
    patterns::checkers::Checkers,
    shapes::{plane::Plane, sphere::Sphere},
    tuple::Tuple,
    world::World,
};

fn main() -> std::io::Result<()> {
    let floor = Plane::default().set_material(
        Material::default()
            .set_color(Color::new(1., 0.9, 0.9))
            .set_specular(0.2)
            .set_reflective(0.4)
            .set_pattern(
                Checkers::new(RGB::new(160, 86, 0).into(), RGB::new(24, 24, 24).into()).into(),
            ),
    );

    let sphere = Sphere::default()
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
                .translation(0., 0.5, 0.),
        );

    let world = World::new(
        Some(Light::new(
            Tuple::point(-10., 10., -8.),
            Color::new(1., 1., 1.),
        )),
        vec![Box::new(sphere), Box::new(floor)],
    );

    // 4K - 3840 × 2160
    // 8K - 7680 × 4320

    let camera = Camera::new(1500, 1000, PI / 3.).set_transform(Matrix::identity().view_transform(
        Tuple::point(0., 1.5, -5.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);

    let img = image::load_from_memory(&canvas.to_ppm().as_bytes()).unwrap();

    img.save("scene.png").unwrap();

    Ok(())
}
