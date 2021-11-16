use ray_tracer::tuple::Tuple;

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

fn main() {
    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };

    let mut count = 0u32;

    loop {
        count += 1;

        projectile = tick(environment.clone(), projectile.clone());

        println!("tick: {}, projectile: {:#?}", count, &projectile);

        if projectile.position.y <= 0.0 {
            break;
        }
    }
}
