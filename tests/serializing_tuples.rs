use magnus::{eval, Error, RArray, Value};
use serde_magnus::serialize;

#[test]
fn test_serializing_tuples() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: Value = serialize(&())?;
    assert!(output.is_nil());

    let output: RArray = serialize(&(1, 2, 3))?;
    assert!(eval!("output == [ 1, 2, 3 ]", output)?);

    Ok(())
}
