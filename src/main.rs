use std::io::prelude::*;
use std::{f64::consts::PI, fs::File};

use ray_tracer::{canvas::Canvas, color::Color, matrix::Matrix, tuple::Tuple};

#[derive(Debug, Clone)]

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Debug, Clone)]

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let position = proj.position.clone() + proj.velocity.clone();
    let velocity = proj.velocity + env.gravity + env.wind;

    Projectile { position, velocity }
}

fn main() -> std::io::Result<()> {
    let mut canvas = Canvas::new(100, 100);
    let point_color = Color::new(1.0, 0.0, 1.0);
    let scaling = Matrix::identity().scaling(25., 25., 25.);
    let rotation = Matrix::identity().rotation_z(PI / 2.);
    let twelve = scaling * Tuple::point(1., 1., 0.);
    let one = rotation * twelve;
    let two = rotation * one;
    let tree = rotation * two;
    let four = rotation * tree;

    canvas.set_center(twelve.x as usize, twelve.y as usize, &point_color);
    canvas.set_center(one.x as usize, one.y as usize, &point_color);
    canvas.set_center(two.x as usize, two.y as usize, &point_color);
    canvas.set_center(tree.x as usize, tree.y as usize, &point_color);
    canvas.set_center(four.x as usize, four.y as usize, &point_color);

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
