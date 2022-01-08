use serde_yaml::Value;

use ray_tracer_loader::parse_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("world.yaml")?;

    let config: Value = serde_yaml::from_reader(f)?;

    let (camera, world) = parse_config(config)?;

    let canvas = camera.render(world);

    let img = image::load_from_memory(&canvas.to_ppm().as_bytes()).unwrap();

    img.save("scene.png").unwrap();

    Ok(())
}
