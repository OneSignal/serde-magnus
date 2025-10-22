use magnus::{Error, Float};
use serde_magnus::serialize;

#[test]
fn test_serializing_floats() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let float: Float = serialize(&ruby, &1.23)?;
    assert_eq!(1.23, float.to_f64());

    Ok(())
}
