use serde_yaml::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("world.yaml")?;

    let d: Value = serde_yaml::from_reader(f)?;

    match d {
        Value::Null => todo!(),
        Value::Bool(_) => todo!(),
        Value::Number(_) => todo!(),
        Value::String(_) => todo!(),
        Value::Sequence(v) => {
            println!("Read YAML: {:#?}", v);
        }
        Value::Mapping(_) => todo!(),
    };

    Ok(())
}
