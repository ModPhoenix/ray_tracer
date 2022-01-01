use std::f64::consts::PI;

use ray_tracer::{
    camera::Camera,
    color::{Color, RGB},
    light::Light,
    material::Material,
    matrix::Matrix,
    patterns::{checkers::Checkers, gradient::Gradient, Pattern},
    shapes::{cone::Cone, cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere},
    tuple::Tuple,
    world::World,
};

fn main() -> std::io::Result<()> {
    let mirror_material = Material::default()
        .set_color(RGB::new(0, 0, 0).into())
        .set_specular(0.2)
        .set_reflective(0.9);

    let mirror = Cube::default()
        .set_material(mirror_material.clone())
        .set_transform(
            Matrix::identity()
                .scaling(2., 2., 0.1)
                .translation(0., 0.5, 2.),
        );

    let mirror2 = mirror.clone().set_transform(
        Matrix::identity()
            .scaling(2., 2., 0.1)
            .translation(0., 0.5, -6.),
    );

    let floor = Plane::default().set_material(
        Material::default()
            .set_color(Color::new(1., 0.9, 0.9))
            .set_specular(0.2)
            .set_reflective(0.4)
            .set_pattern(
                Checkers::new(RGB::new(160, 86, 0).into(), RGB::new(24, 24, 24).into()).into(),
            ),
    );

    let _cube = Cube::default()
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

    let _cube2 = Cube::default()
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

    let _middle = Sphere::default()
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
                .set_color(RGB::new(255, 192, 203).into())
                // .set_transparency(1.)
                // .set_refractive_index(1.5)
                // .set_ambient(0.1)
                // .set_diffuse(0.1)
                // .set_specular(0.1)
                // .set_shininess(300.),
                .set_reflective(0.4),
        )
        .set_transform(
            Matrix::identity()
                .scaling(0.5, 0.5, 0.5)
                .translation(1.5, 0.5, -0.5),
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
                .scaling(0.2, 0.2, 0.2)
                .translation(0., 0.7, -0.5),
        );

    let _left = Sphere::default()
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

    let cylinder = Cylinder::default()
        .set_minimum(0.)
        .set_maximum(1.)
        .set_closed(true)
        .set_material(
            Material::default()
                .set_color(RGB::new(100, 172, 143).into())
                .set_reflective(0.2),
        )
        .set_transform(
            Matrix::identity()
                .scaling(0.3, 0.8, 0.3)
                .translation(-1.5, 0., -0.5),
        );

    let cone = Cone::default()
        .set_minimum(0.)
        .set_maximum(1.)
        .set_closed(true)
        .set_material(
            Material::default().set_pattern(
                Checkers::new(
                    RGB::new(169, 109, 142).into(),
                    RGB::new(187, 202, 204).into(),
                )
                .into(),
            ),
        )
        .set_transform(
            Matrix::identity()
                .scaling(0.5, 0.5, 0.5)
                .translation(0., 0., -0.5),
        );

    let world = World::new(
        Some(Light::new(
            Tuple::point(-10., 10., -8.),
            Color::new(1., 1., 1.),
        )),
        vec![
            Box::new(mirror),
            Box::new(mirror2),
            Box::new(floor),
            Box::new(sky),
            Box::new(middle2),
            Box::new(right),
            Box::new(cylinder),
            Box::new(cone),
        ],
    );

    // 4K - 3840 × 2160
    // 8K - 7680 × 4320

    let camera = Camera::new(3840, 2160, PI / 3.).set_transform(Matrix::identity().view_transform(
        Tuple::point(-2., 1.5, -5.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    ));

    let canvas = camera.render(world);

    let img = image::load_from_memory(&canvas.to_ppm().as_bytes()).unwrap();

    img.save("scene.png").unwrap();

    Ok(())
}
