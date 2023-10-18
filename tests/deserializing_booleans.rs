use magnus::{
    value::{qfalse, qtrue, ReprValue},
    Error,
};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_booleans() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: bool = deserialize(qtrue().as_value())?;
    assert_eq!(true, output);

    let output: bool = deserialize(qfalse().as_value())?;
    assert_eq!(false, output);

    Ok(())
}
