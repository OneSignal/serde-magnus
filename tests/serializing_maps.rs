use magnus::{Error, RHash, RString};
use serde_magnus::serialize;
use std::collections::HashMap;

#[test]
fn test_serializing_maps() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let mut input: HashMap<&str, &str> = HashMap::new();
    input.insert("Yes", "No");
    input.insert("Stop", "Go");
    input.insert("High", "Low");

    let output: RHash = serialize(&input)?;
    assert_eq!(3, output.len());

    let value: RString = output.lookup("Yes")?;
    assert_eq!("No", value.to_string()?);

    let value: RString = output.lookup("Stop")?;
    assert_eq!("Go", value.to_string()?);

    let value: RString = output.lookup("High")?;
    assert_eq!("Low", value.to_string()?);

    Ok(())
}
