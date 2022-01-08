use serde_yaml::{Mapping, Value};

pub fn get_value_by_key<'a>(config: &'a Mapping, key: &str) -> Option<&'a Value> {
    config.get(&Value::String(key.to_string()))
}

pub fn get_vec_f64_from_sequence<'a>(config: &'a Mapping, key: &str) -> Option<Vec<f64>> {
    let value = get_value_by_key(config, key)?;

    Some(
        value
            .as_sequence()?
            .iter()
            .map(|item| item.as_f64().unwrap())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use serde_yaml::Value;

    use crate::utils::{get_value_by_key, get_vec_f64_from_sequence};

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
}
