use magnus::{eval, Error, Value};
use serde_magnus::serialize;

#[test]
fn test_serializing_booleans() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: Value = serialize(&true)?;
    assert!(eval!("output == true", output)?);

    let output: Value = serialize(&false)?;
    assert!(eval!("output == false", output)?);

    Ok(())
}
