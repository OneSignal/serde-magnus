use magnus::{Error, Float};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_floats() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: f64 = deserialize(Float::from_f64(1.23))?;
    assert_eq!(1.23, output);

    Ok(())
}
