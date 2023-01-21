use magnus::{Error, Integer, RArray, QNIL};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_tuples() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    assert_eq!((), deserialize(QNIL)?);

    let input: RArray = RArray::new();
    input.push(Integer::from_i64(1))?;
    input.push(Integer::from_i64(2))?;
    input.push(Integer::from_i64(3))?;

    let output: (i64, i64, i64) = deserialize(input)?;
    assert_eq!((1, 2, 3), output);

    Ok(())
}
