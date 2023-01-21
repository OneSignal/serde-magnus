use magnus::{Error, Float};
use serde_magnus::serialize;

#[test]
fn test_serializing_floats() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let float: Float = serialize(&1.23)?;
    assert_eq!(1.23, float.to_f64());

    Ok(())
}
