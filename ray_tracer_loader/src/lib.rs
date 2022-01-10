use anyhow::{Context, Result};

use ray_tracer::{
    camera::Camera,
    color::Color,
    light::Light,
    material::Material,
    matrix::Matrix,
    shapes::{cone::Cone, cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere, Shape},
    tuple::Tuple,
    world::World,
};
use serde_yaml::{Mapping, Value};
use utils::{get_material, get_value_by_key, get_vec_f64_from_sequence};

use crate::utils::get_transform;

mod utils;

pub fn parse_config(config: Value) -> Result<(Camera, World)> {
    let mut camera = None;
    let mut light = None;
    let mut objects: Vec<Box<dyn Shape>> = vec![];

    for value in config
        .as_sequence()
        .context("config should be a sequence")?
    {
        if let Value::Mapping(command) = value {
            if let Some(item) = get_value_by_key(&command, "add") {
                if let Value::String(object) = item {
                    match object.as_str() {
                        "camera" => {
                            camera = get_camera_from_config(&command);
                        }
                        "light" => {
                            light = get_light_from_config(&command);
                        }
                        _ => {
                            objects.push(
                                get_shape_from_config(&command)
                                    .context("Can't parse shape from config")?,
                            );
                        }
                    }
                }
            }
        }
    }

    let world = World::new(Some(light.context("Light is required")?), objects);

    Ok((camera.context("Camera is required")?, world))
}

fn get_camera_from_config(config: &Mapping) -> Option<Camera> {
    let width = get_value_by_key(config, "width")?.as_i64()?;
    let height = get_value_by_key(config, "height")?.as_i64()?;
    let field_of_view = get_value_by_key(config, "field-of-view")?.as_f64()?;
    let from = get_vec_f64_from_sequence(config, "from")?;
    let to = get_vec_f64_from_sequence(config, "to")?;
    let up = get_vec_f64_from_sequence(config, "up")?;

    Some(
        Camera::new(width as usize, height as usize, field_of_view).set_transform(
            Matrix::identity().view_transform(
                Tuple::point(from[0], from[1], from[2]),
                Tuple::point(to[0], to[1], to[2]),
                Tuple::vector(up[0], up[1], up[2]),
            ),
        ),
    )
}

fn get_light_from_config(config: &Mapping) -> Option<Light> {
    let position = get_vec_f64_from_sequence(config, "at")?;
    let intensity = get_vec_f64_from_sequence(config, "intensity")?;

    Some(Light::new(
        Tuple::point(position[0], position[1], position[2]),
        Color::new(intensity[0], intensity[1], intensity[2]),
    ))
}

fn generate_shape<T: Shape + Default>(
    transform: Option<Matrix<4>>,
    material: Option<Material>,
) -> T {
    let mut shape = T::default();

    if let Some(transform) = transform {
        shape.set_transform(transform);
    }

    if let Some(material) = material {
        shape.set_material(material);
    }

    shape
}

fn get_shape_from_config(config: &Mapping) -> Option<Box<dyn Shape>> {
    let variant = get_value_by_key(config, "add")?.as_str()?;
    let transform = get_transform(config);
    let material = get_material(config);

    let shape: Option<Box<dyn Shape>> = match variant {
        "sphere" => Some(Box::new(generate_shape::<Sphere>(transform, material))),
        "plane" => Some(Box::new(generate_shape::<Plane>(transform, material))),
        "cube" => Some(Box::new(generate_shape::<Cube>(transform, material))),
        "cylinder" => Some(Box::new(generate_shape::<Cylinder>(transform, material))),
        "cone" => Some(Box::new(generate_shape::<Cone>(transform, material))),
        _ => {
            println!("miss variant: {}", variant);
            None
        }
    };

    shape
}

#[cfg(test)]
mod tests {
    use ray_tracer::{camera::Camera, color::Color, light::Light, matrix::Matrix, tuple::Tuple};
    use serde_yaml::Value;

    use crate::{get_camera_from_config, parse_config};

    #[test]
    fn parse_config_should_return_camera_and_world() {
        let yaml = r#"
  - add: camera
    width: 400
    height: 160
    field-of-view: 0.7854
    from: [-3, 1, 2.5]
    to: [0, 0.5, 0]
    up: [0, 1, 0]

  - add: light
    at: [-4.9, 4.9, -1]
    intensity: [1, 1, 1]

  - add: plane
    material:
      color: [1, 1, 1]
      ambient: 0.025
      diffuse: 0.67
      specular: 0

  - add: sphere
    transform:
      - [scale, 0.4, 0.4, 0.4]
      - [translate, 4.6, 0.4, 1]
    material:
      color: [0.8, 0.5, 0.3]
      shininess: 50

  - add: cube
    transform:
      - [scale, 0.4, 0.4, 0.4]
      - [translate, 4.6, 0.4, 1]
    material:
      color: [0.8, 0.5, 0.3]
      shininess: 50"#;

        let config: Value = serde_yaml::from_str(yaml).unwrap();
        let (camera, world) = parse_config(config).unwrap();

        let expected_camera =
            Camera::new(400, 160, 0.7854).set_transform(Matrix::identity().view_transform(
                Tuple::point(-3., 1., 2.5),
                Tuple::point(0., 0.5, 0.),
                Tuple::vector(0., 1., 0.),
            ));

        let expected_light = Light::new(Tuple::point(-4.9, 4.9, -1.), Color::new(1., 1., 1.));

        assert_eq!(camera, expected_camera);
        assert_eq!(world.light(), Some(&expected_light));
        assert_eq!(world.objects().len(), 3);
    }

    #[test]
    fn get_camera_should_return_a_camera_from_config() {
        let yaml = r#"
add: camera
width: 400
height: 160
field-of-view: 0.7854
from: [-3, 1, 2.5]
to: [0, 0.5, 0]
up: [0, 1, 0]"#;

        let config: Value = serde_yaml::from_str(yaml).unwrap();
        let camera_config = config.as_mapping().unwrap();

        let result = get_camera_from_config(camera_config);

        assert_eq!(
            result,
            Some(
                Camera::new(400, 160, 0.7854).set_transform(Matrix::identity().view_transform(
                    Tuple::point(-3., 1., 2.5),
                    Tuple::point(0., 0.5, 0.),
                    Tuple::vector(0., 1., 0.),
                ))
            )
        );
    }
}
