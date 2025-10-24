use magnus::{eval, Error, RHash};
use serde_magnus::serialize;
use std::collections::HashMap;

#[test]
fn test_serializing_maps() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let mut input: HashMap<&str, &str> = HashMap::new();
    input.insert("Yes", "No");
    input.insert("Stop", "Go");
    input.insert("High", "Low");

    let output: RHash = serialize(&ruby, &input)?;

    assert!(eval!(
        &ruby,
        r#"
            output == {
              "Yes" => "No",
              "Stop" => "Go",
              "High" => "Low"
            }
        "#,
        output
    )?);

    Ok(())
}
