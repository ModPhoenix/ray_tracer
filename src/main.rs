use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

use ray_tracer::intersections::{Intersectable, Intersections};
use ray_tracer::light::Light;
use ray_tracer::material::Material;
use ray_tracer::matrix::Matrix;
use ray_tracer::ray::Ray;
use ray_tracer::sphere::Sphere;
use ray_tracer::world::Normal;
use ray_tracer::{canvas::Canvas, color::Color, tuple::Tuple};

fn main() -> std::io::Result<()> {
    let canvas_pixels = 1000;
    let ray_origin = Tuple::point(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let sphere = Sphere::default()
        .set_material(Material::default().set_color(Color::new(1., 1.0, 1.0)))
        .set_transform(
            Matrix::identity()
                .scaling(1., 0.6, 0.6)
                .rotation_y(PI / 1.5)
                .rotation_z(PI / 1.5),
        );

    let light_position = Tuple::point(-10., 10., -15.);
    let light_color = Color::new(1., 1., 1.);
    let light = Light::new(light_position, light_color);

    for y in 0..canvas.height {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = sphere.intersect(&ray);

            if let Some(intersect) = xs {
                let intersections = Intersections::new(intersect.to_vec());

                if let Some(hit) = intersections.hit() {
                    let point = ray.position(hit.t);
                    let normal = hit.object.normal_at(point);
                    let eye = -ray.direction;

                    let color =
                        hit.object
                            .material()
                            .clone()
                            .lighting(light.clone(), point, eye, normal);

                    canvas.set(x, y, &color);
                }
            }
        }
    }

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
