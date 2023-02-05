use magnus::{eval, Error, RArray, QNIL};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_tuples() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    assert_eq!((), deserialize(QNIL)?);

    let input: RArray = eval!("[ 123, true, 'Hello, world!' ]")?;
    let output: (i64, bool, String) = deserialize(input)?;
    assert_eq!((123, true, "Hello, world!".into()), output);

    Ok(())
}
