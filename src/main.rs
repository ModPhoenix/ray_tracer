use std::fs::File;
use std::io::prelude::*;

use ray_tracer::intersections::Intersectable;
use ray_tracer::matrix::Matrix;
use ray_tracer::ray::Ray;
use ray_tracer::sphere::Sphere;
use ray_tracer::{canvas::Canvas, color::Color, tuple::Tuple};

fn main() -> std::io::Result<()> {
    let canvas_pixels = 100;
    let ray_origin = Tuple::point(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let point_color = Color::new(1.0, 0.0, 1.0);
    let mut shape = Sphere::new();

    shape.set_transform(
        Matrix::identity()
            .scaling(0.5, 1., 1.)
            .shearing(1., 0., 0., 0., 0., 0.),
    );

    for y in 0..canvas.height {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&ray);

            match xs {
                Some(_) => {
                    canvas.set(x, y, &point_color);
                }
                None => {}
            }
        }
    }

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
