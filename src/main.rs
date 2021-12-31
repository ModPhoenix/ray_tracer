use std::f64::consts::PI;

use ray_tracer::{
    camera::Camera, color::Color, light::Light, material::Material, matrix::Matrix,
    shapes::sphere::Sphere, tuple::Tuple, world::World,
};

fn main() -> std::io::Result<()> {
    let sphere = Sphere::default()
        .set_material(Material::default().set_color(Color::new(1., 0., 0.2)))
        .set_transform(Matrix::identity().scaling(0.2, 0.2, 0.2));

    let world = World::new(
        Some(Light::new(
            Tuple::point(-10., 10., -8.),
            Color::new(1., 1., 1.),
        )),
        vec![Box::new(sphere)],
    );

    // 4K - 3840 × 2160
    // 8K - 7680 × 4320

    let camera = Camera::new(1500, 1000, PI / 3.).set_transform(Matrix::identity().view_transform(
        Tuple::point(0., 1.5, -3.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);

    let img = image::load_from_memory(&canvas.to_ppm().as_bytes()).unwrap();

    img.save("scene.png").unwrap();

    Ok(())
}
