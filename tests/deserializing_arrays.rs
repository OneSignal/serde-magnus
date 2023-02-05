use magnus::{eval, Error, RArray};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_arrays() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let input: RArray = eval!("[1, 2, 3]")?;
    let output: [i64; 3] = deserialize(input)?;
    assert_eq!([1, 2, 3], output);

    Ok(())
}
