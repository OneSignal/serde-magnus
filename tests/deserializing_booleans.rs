use magnus::{Error, QFALSE, QTRUE};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_booleans() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: bool = deserialize(QTRUE)?;
    assert_eq!(true, output);

    let output: bool = deserialize(QFALSE)?;
    assert_eq!(false, output);

    Ok(())
}
