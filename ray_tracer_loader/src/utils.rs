use ray_tracer::{
    color::Color,
    material::Material,
    matrix::Matrix,
    patterns::{checkers::Checkers, gradient::Gradient, ring::Ring, stripe::Stripe, Patterns},
};
use serde_yaml::{Mapping, Value};

pub fn get_value_by_key<'a>(config: &'a Mapping, key: &str) -> Option<&'a Value> {
    config.get(&Value::String(key.to_string()))
}

pub fn as_vec_f64(sequence: &Vec<Value>) -> Option<Vec<f64>> {
    Some(sequence.iter().map(|item| item.as_f64().unwrap()).collect())
}

pub fn get_vec_f64_from_sequence<'a>(config: &'a Mapping, key: &str) -> Option<Vec<f64>> {
    let sequence = get_value_by_key(config, key)?.as_sequence()?;

    Some(as_vec_f64(sequence)?)
}

fn get_translate_args(sequence: &Vec<Value>) -> Option<(&str, Vec<f64>)> {
    let trans_type = sequence.get(0)?.as_str()?;
    let args = as_vec_f64(&sequence[1..].into())?;

    Some((trans_type, args))
}

pub fn get_transform(shape_config: &Mapping) -> Option<Matrix<4>> {
    let transform = get_value_by_key(shape_config, "transform")?.as_sequence()?;

    let mut matrix = Matrix::identity();

    for value in transform {
        let sequence = value.as_sequence()?;
        let (trans_type, args) = get_translate_args(sequence)?;

        match trans_type {
            "translate" => {
                matrix = matrix.translation(args[0], args[1], args[2]);
            }
            "scale" => {
                matrix = matrix.scaling(args[0], args[1], args[2]);
            }
            "rotate-x" => {
                matrix = matrix.rotation_x(args[0]);
            }
            "rotate-y" => {
                matrix = matrix.rotation_y(args[0]);
            }
            "rotate-z" => {
                matrix = matrix.rotation_z(args[0]);
            }
            "shearing" => {
                matrix = matrix.shearing(args[0], args[1], args[2], args[3], args[4], args[5]);
            }
            _ => {}
        }
    }

    Some(matrix)
}

fn get_pattern(pattern_config: &Mapping) -> Option<Patterns> {
    let pattern_type = get_value_by_key(pattern_config, "type")?.as_str()?;
    let colors = get_value_by_key(pattern_config, "colors")?.as_sequence()?;
    let color1 = as_vec_f64(colors[0].as_sequence()?)?;
    let color2 = as_vec_f64(colors[1].as_sequence()?)?;

    match pattern_type {
        "checkers" => Some(
            Checkers::new(
                Color::new(color1[0], color1[1], color1[2]),
                Color::new(color2[0], color2[1], color2[2]),
            )
            .into(),
        ),
        "gradient" => Some(
            Gradient::new(
                Color::new(color1[0], color1[1], color1[2]),
                Color::new(color2[0], color2[1], color2[2]),
            )
            .into(),
        ),
        "ring" => Some(
            Ring::new(
                Color::new(color1[0], color1[1], color1[2]),
                Color::new(color2[0], color2[1], color2[2]),
            )
            .into(),
        ),
        "stripe" => Some(
            Stripe::new(
                Color::new(color1[0], color1[1], color1[2]),
                Color::new(color2[0], color2[1], color2[2]),
            )
            .into(),
        ),
        _ => None,
    }
}

pub fn get_material(shape_config: &Mapping) -> Option<Material> {
    let mapping = get_value_by_key(shape_config, "material")?.as_mapping()?;

    let mut material = Material::default();

    for (key, value) in mapping.iter() {
        match key.as_str()? {
            "pattern" => {
                let pattern = get_pattern(value.as_mapping()?)?;

                material = material.set_pattern(pattern);
            }
            "color" => {
                let color = as_vec_f64(value.as_sequence()?)?;
                material = material.set_color(Color::new(color[0], color[1], color[2]));
            }
            "ambient" => {
                material = material.set_ambient(value.as_f64()?);
            }
            "diffuse" => {
                material = material.set_diffuse(value.as_f64()?);
            }
            "specular" => {
                material = material.set_specular(value.as_f64()?);
            }
            "shininess" => {
                material = material.set_shininess(value.as_f64()?);
            }
            "reflective" => {
                material = material.set_reflective(value.as_f64()?);
            }
            "transparency" => {
                material = material.set_transparency(value.as_f64()?);
            }
            "refractive-index" => {
                material = material.set_refractive_index(value.as_f64()?);
            }
            _ => {}
        }
    }

    Some(material)
}

#[cfg(test)]
mod tests {
    use ray_tracer::{
        color::Color, material::Material, matrix::Matrix, patterns::checkers::Checkers,
    };
    use serde_yaml::Value;

    use crate::utils::{get_material, get_transform, get_value_by_key, get_vec_f64_from_sequence};

    #[test]
    fn get_value_by_key_works() {
        let yaml = r#"add: camera
width: 400
height: 160
field-of-view: 0.7854
from: [-3, 1, 2.5]
to: [0, 0.5, 0]
up: [0, 1, 0]"#;

        let config: Value = serde_yaml::from_str(yaml).unwrap();
        let camera_config = config.as_mapping().unwrap();

        let result1 = get_value_by_key(camera_config, "width");
        let result2 = get_value_by_key(camera_config, "not-found");

        assert_eq!(result1.unwrap().as_i64(), Some(400));
        assert!(result2.is_none());
    }

    #[test]
    fn get_vec_f64_from_sequence_works() {
        let yaml = r#"add: camera
width: 400
height: 160
field-of-view: 0.7854
from: [-3, 1, 2.5]
to: [0, 0.5, 0]
up: [0, 1, 0]"#;

        let config: Value = serde_yaml::from_str(yaml).unwrap();
        let camera_config = config.as_mapping().unwrap();

        let result1 = get_vec_f64_from_sequence(camera_config, "from");
        let result2 = get_vec_f64_from_sequence(camera_config, "not-found");

        assert_eq!(result1, Some(vec![-3., 1., 2.5]));
        assert!(result2.is_none());
    }

    #[test]
    fn get_transform_works_if_transform_in_config() {
        let yaml = r#"
add: plane
transform:
  - [rotate-y, 1.5708] # orient texture
  - [rotate-z, 1.5708] # rotate to vertical
  - [scale, 0.3, 0.8, 0.3]
  - [translate, 5, 0, 0]"#;

        let config: Value = serde_yaml::from_str(yaml).unwrap();
        let config_mapping = config.as_mapping().unwrap();

        let result = get_transform(config_mapping);

        assert_eq!(
            result,
            Some(
                Matrix::identity()
                    .rotation_y(1.5708)
                    .rotation_z(1.5708)
                    .scaling(0.3, 0.8, 0.3)
                    .translation(5., 0., 0.)
            )
        );
    }

    #[test]
    fn get_transform_return_none_if_transform_not_found() {
        let yaml = r#"add: plane"#;

        let config: Value = serde_yaml::from_str(yaml).unwrap();
        let config_mapping = config.as_mapping().unwrap();

        let result = get_transform(config_mapping);

        assert!(result.is_none());
    }

    #[test]
    fn get_material_works() {
        let yaml = r#"
add: sphere
transform:
    - [scale, 0.5, 0.5, 0.5]
    - [translate, -0.7, 0.5, -0.8]
material:
    color: [0, 0.2, 0]
    ambient: 0
    diffuse: 0.4
    specular: 0.9
    shininess: 300
    reflective: 0.9
    transparency: 0.9
    refractive-index: 1.5"#;

        let config: Value = serde_yaml::from_str(yaml).unwrap();
        let config_mapping = config.as_mapping().unwrap();

        let result = get_material(config_mapping);

        assert_eq!(
            result,
            Some(
                Material::default()
                    .set_color(Color::new(0., 0.2, 0.))
                    .set_ambient(0.)
                    .set_diffuse(0.4)
                    .set_specular(0.9)
                    .set_shininess(300.)
                    .set_reflective(0.9)
                    .set_transparency(0.9)
                    .set_refractive_index(1.5)
            )
        );
    }

    #[test]
    fn get_material_with_pattern_works() {
        let yaml = r#"
add: sphere
transform:
    - [scale, 0.5, 0.5, 0.5]
    - [translate, -0.7, 0.5, -0.8]
material:
    pattern:
        type: checkers
        colors:
        - [0.35, 0.35, 0.35]
        - [0.65, 0.65, 0.65]
    specular: 0
    reflective: 0.4"#;

        let config: Value = serde_yaml::from_str(yaml).unwrap();
        let config_mapping = config.as_mapping().unwrap();

        let result = get_material(config_mapping);

        assert_eq!(
            result,
            Some(
                Material::default()
                    .set_specular(0.)
                    .set_reflective(0.4)
                    .set_pattern(
                        Checkers::new(Color::new(0.35, 0.35, 0.35), Color::new(0.65, 0.65, 0.65))
                            .into()
                    )
            )
        );
    }
}
