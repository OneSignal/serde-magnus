use magnus::{Error, Integer, RArray, RString, QNIL, QTRUE};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_tuples() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    assert_eq!((), deserialize(QNIL)?);

    let input: RArray = RArray::new();
    input.push(Integer::from_i64(123))?;
    input.push(QTRUE)?;
    input.push(RString::from("Hello, world!"))?;

    let output: (i64, bool, String) = deserialize(input)?;
    assert_eq!((123, true, "Hello, world!".into()), output);

    Ok(())
}
