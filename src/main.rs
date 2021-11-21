use std::fs::File;
use std::io::prelude::*;

use ray_tracer::{canvas::Canvas, color::Color, tuple::Tuple};

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
    let mut canvas = Canvas::new(900, 550);
    let point_color = Color::new(1.0, 0.0, 1.0);
    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.05, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize() * 11.25,
    };

    loop {
        projectile = tick(environment.clone(), projectile.clone());

        if projectile.position.y <= 0.0 || projectile.position.x as usize >= canvas.width {
            break;
        }

        println!("position.y: {}", projectile.position.y);
        println!("position.x: {}", projectile.position.x);

        canvas.set(
            canvas.width - projectile.position.x.clone() as usize,
            canvas.height - projectile.position.y.clone() as usize,
            &point_color,
        );
    }

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
