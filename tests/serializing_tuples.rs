use magnus::{eval, Error, RArray, Value};
use serde_magnus::serialize;

#[test]
fn test_serializing_tuples() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    // To nil
    let output: Value = serialize(&())?;
    assert!(output.is_nil());

    // To a homogeneous array
    let output: RArray = serialize(&(1, 2, 3))?;
    assert!(eval!("output == [ 1, 2, 3 ]", output)?);

    // To a heterogeneous array
    let input: (u64, bool, &str) = (1234, true, "Hello, world!");
    let output: RArray = serialize(&input)?;
    assert!(eval!("output == [ 1234, true, 'Hello, world!' ]", output)?);

    // To an array containing nil
    let input: (u64, (), &str) = (1234, (), "Hello, world!");
    let output: RArray = serialize(&input)?;
    assert!(eval!("output == [ 1234, nil, 'Hello, world!' ]", output)?);

    Ok(())
}
