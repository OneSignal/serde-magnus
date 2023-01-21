use magnus::{Error, Integer, RArray};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_arrays() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let input: RArray = RArray::new();
    input.push(Integer::from_i64(1))?;
    input.push(Integer::from_i64(2))?;
    input.push(Integer::from_i64(3))?;

    let output: [i64; 3] = deserialize(input)?;
    assert_eq!([1, 2, 3], output);

    Ok(())
}
