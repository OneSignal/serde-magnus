use magnus::{
    eval,
    value::{qnil, ReprValue},
    Error, RArray,
};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_tuples() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    // From nil
    assert_eq!((), deserialize(qnil().as_value())?);

    // From a homogeneous array
    let input: RArray = eval!("[ 123, 456, 789 ]")?;
    let output: (i64, i64, i64) = deserialize(input)?;
    assert_eq!((123, 456, 789), output);

    // From a heterogeneous array
    let input: RArray = eval!("[ 123, true, 'Hello, world!' ]")?;
    let output: (i64, bool, String) = deserialize(input)?;
    assert_eq!((123, true, "Hello, world!".into()), output);

    // From an array containing nil
    let input: RArray = eval!("[ 123, nil, 'Hello, world!' ]")?;
    let output: (i64, (), String) = deserialize(input)?;
    assert_eq!((123, (), "Hello, world!".into()), output);

    Ok(())
}
